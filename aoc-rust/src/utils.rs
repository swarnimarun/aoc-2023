use std::path::{Path, PathBuf};

#[macro_export]
macro_rules! get_file_name {
    () => {{
        let p = std::path::PathBuf::from(std::file!());
        let s = p.file_stem().unwrap_or_default();
        s.to_string_lossy().to_string()
    }};
}

#[macro_export]
macro_rules! aoc_input {
    () => {{
        crate::utils::read_input(get_file_name!())
    }};
}

pub fn read_input(p: impl AsRef<Path>) -> String {
    let path = PathBuf::from("../.aocdata").join(p.as_ref());
    std::fs::read_to_string(&path).expect(&format!("Failed to read: {}", path.display()))
}
