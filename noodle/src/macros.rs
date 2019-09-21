#[macro_export]
macro_rules! impl_comptr {
    ($name:ident: [$com:ty, $($extra:ty),*]) => {
        //pub struct $name {
        //    native: std::cell::UnsafeCell<$crate::comptr::ComPtr<$com>>
        //}
        pub type $name = $crate::comptr::ComPtr<$com>;

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
               self.as_ptr()  as *mut $int
            }
        }
    }
}
