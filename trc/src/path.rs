use std::path::{Path, PathBuf};

pub trait IntoSlash {
    fn into_slash(self) -> PathBuf;
}

pub trait IntoBackSlash {
    fn into_backslash(self) -> PathBuf;
}

impl IntoSlash for PathBuf {
    fn into_slash(self) -> PathBuf {
        let path =
            String::from_utf8_lossy(self.to_path_buf().to_string_lossy().as_bytes()).to_string();
        let path = path.replace("\\", "/");
        let mut pathbuf = PathBuf::new();
        pathbuf.set_file_name(path);
        pathbuf
    }
}

impl IntoBackSlash for PathBuf {
    fn into_backslash(self) -> PathBuf {
        let path =
            String::from_utf8_lossy(self.to_path_buf().to_string_lossy().as_bytes()).to_string();
        let path = path.replace("/", "\\");
        let mut pathbuf = PathBuf::new();
        pathbuf.set_file_name(path);
        pathbuf
    }
}
