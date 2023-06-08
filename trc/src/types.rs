#[cfg(target_os = "windows")]
pub mod windows {
    use std::ffi::c_char;

    pub type wchar_t = u16

    pub type CHAR = c_char;
    pub type WCHAR = wchar_t;
    pub type LPCSTR = *const CHAR;
    pub type LPSTR = *mut CHAR;
    pub type PSTR = *mut CHAR;
    pub type LPCWSTR = *const WCHAR;
    pub type PWSTR = *mut WCHAR;
    pub type LPWSTR = *mut WCHAR;
}
