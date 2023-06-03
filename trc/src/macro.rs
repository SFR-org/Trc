#[macro_export]
macro_rules! ref_ptr {
    ( $( $x:expr ),* ) => {
        {
            $(
                unsafe {
                    &*$x
                }
            )*
        }
    };
}