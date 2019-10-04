#[macro_export]
macro_rules! impl_comptr {
    ($name:ident: [$com:ty, $($extra:ty),*]) => {
        #[derive(Default)]
        pub struct $name(crate $crate::comptr::ComPtr<$com>);

        //pub type $name = $crate::comptr::ComPtr<$com>;

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("ptr", &self.0.as_ptr())
                    .finish()
            }
        }

        impl From<$crate::ComPtr<$com>> for $name {
            fn from(comptr: $crate::ComPtr<$com>) -> Self {
                Self(comptr)
            }
        }

        impl $name {
            pub fn as_<Q,I>(&self) -> Option<Q>
            where
                Q: From<$crate::ComPtr<I>>,
                I: winapi::Interface,
            {
                self.0.query_interface().map(|qi| Q::from(qi))
            }
        }

        impl_comptr! { @interface $name $com }
        $(impl_comptr! { @interface $name $extra })*
    };
    (@interface $name:ident $int:ty) => {
        // unsafe impl $crate::AsInterface<$int> for $name {
        //     unsafe fn raw_ptr(&self) -> *mut $int {
        //         (&*self.native.get()).as_ptr() as
        //     }
        // }
        impl $crate::AsPtr<$int> for $name {
            fn as_ptr(&self) -> *mut $int {
               self.0.as_ptr()  as *mut $int
            }
        }
    }
}

#[macro_export]
macro_rules! impl_interface {
    (impl $name:ident $tt:tt
    ) => {
        impl $name
            $tt
    };
    (impl [$($name:ident),*] $tt:tt
    ) => {
        $(impl $name
            $tt
        )*
    };
}
