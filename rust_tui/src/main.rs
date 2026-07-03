use std::path::{self, PathBuf};
struct AppState{
    current_dir: std::path::PathBuf, //Dir = Directory
    dir_files: Vec<FileItem>,
    cursor : usize
}

struct FileItem{
    name: String,
    path: std::path::PathBuf,
    is_dir: bool,
    size: u64
}

impl AppState{

}


fn main() {
    println!("Hello, world!");
}
