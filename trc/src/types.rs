#[cfg(target_os = "windows")]
pub mod windows {
    use std::ffi::c_char;

    pub type LPCSTR = *const c_char;
    pub type LPCWSTR = *const u16;
}
