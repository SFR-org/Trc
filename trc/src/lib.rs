mod path;



#[cfg(test)]
mod tests {
    use std::path::Path;
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
}
