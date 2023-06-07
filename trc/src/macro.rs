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

#[macro_export]
macro_rules! assert_null {
    ($ptr:expr) => {
        if $ptr == unsafe { std::mem::zeroed() } {
            panic!("Passed pointer is null!");
        }
    };
}

#[cfg(target_os = "windows")]
pub mod windows {
    #[macro_export]
    macro_rules! LPCSTR {
        ( $x:expr ) => {
            std::ffi::CString::new($x).unwrap().as_ptr() as LPCSTR
        };
    }

    #[macro_export]
    macro_rules! LPCWSTR {
        ( $x: expr) => {
            std::ffi::OsString::from($x)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect::<Vec<u16>>()
                .as_ptr() as LPCWSTR
        };
    }
}
