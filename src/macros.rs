
#[macro_export]
macro_rules! hr {
    ($hr:expr) => {
        {
            let hr = unsafe { $hr };
            assert!(hr == 0);
        }
    }
}

#[macro_export]
macro_rules! impl_comptr {
    ($name:ident: [$com:ty, $($extra:ty),*]) => {
        #[derive(Clone, Default)]
        pub struct $name(crate $crate::comptr::ComPtr<$com>);

        impl From<$crate::ComPtr<$com>> for $name {
            fn from(comptr: $crate::ComPtr<$com>) -> Self {
                Self(comptr)
            }
        }

        impl_comptr! { @impl $name: [$com, $($extra),*] }
    };
    ($name:ident($field:ty): [$com:ty, $($extra:ty),*]) => {
        #[derive(Clone, Default)]
        pub struct $name(crate $crate::comptr::ComPtr<$com>, $field);

        impl From<($crate::ComPtr<$com>, $field)> for $name {
            fn from(tuple: ($crate::ComPtr<$com>, $field)) -> Self {
                Self(tuple.0, tuple.1)
            }
        }

        impl_comptr! { @impl $name: [$com, $($extra),*] }
    };
    (@impl $name:ident: [$com:ty, $($extra:ty),*]) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("ptr", &self.0.as_ptr())
                    .finish()
            }
        }

        impl $name {
            #[allow(dead_code)]
            pub fn as_<Q,I>(&self) -> Option<Q>
            where
                Q: From<$crate::ComPtr<I>>,
                I: winapi::Interface,
            {
                self.0.query_interface().map(Q::from)
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

#[macro_export]
macro_rules! impl_newtype {
    ($(pub struct $name:ident($base:ty);)*) => {
        $(#[derive(Clone, Copy, Default, NewType)]
        #[repr(transparent)]
        pub struct $name($base);

        )*
    }
}