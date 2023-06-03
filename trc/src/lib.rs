use std::ffi::c_ulong;

pub mod path;
pub mod r#macro;

pub type HWND = isize;

pub type XLibWndHandle = c_ulong;

pub enum WindowHandle {
    Windows(WindowsHandle),
    XLib(XLibHandle),
}

pub struct XLibHandle {
    handle: XLibWndHandle
}

pub struct WindowsHandle {
    handle: HWND
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::ptr::{addr_of, addr_of_mut};
    use crate::path::{IntoBackSlash, IntoSlash};
    use super::*;

    #[test]
    fn backslash_path_to_slash_path() {
        let path = Path::new("C:\\Users\\Test");
        let path = path.to_owned();
        assert_eq!("C:/Users/Test", path.into_slash().to_str().unwrap());
    }

    #[test]
    fn slash_path_to_backslash_path() {
        let path = Path::new("C:/Users/Test");
        let path = path.to_owned();
        assert_eq!("C:\\Users\\Test", path.into_backslash().to_str().unwrap());
    }

    #[test]
    fn ref_ptr_macro() {
        let mut a = 50;
        let a_ptr = &mut a;
        let b = ref_ptr!(a_ptr);
        assert_eq!(50,*b);
    }
}
