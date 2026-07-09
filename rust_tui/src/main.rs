use std::fs;
use std::path::{self, PathBuf};
use std::env::{self, current_dir};
pub struct AppState{
    pub dir_path: std::path::PathBuf, //Dir = Directory
    pub dir_files: Vec<FileItem>,
    pub cursor : usize
}

struct FileItem{
    name: String,
    path: std::path::PathBuf,
    is_dir: bool,
    size: u64
}

impl AppState{
    pub fn new_state() -> Self{
        AppState{
            dir_path: std::env::current_dir().unwrap(),
            dir_files: Vec::new(),
            cursor: 0
        }
    }

    pub fn scan_current_dir(&mut self){
        std::fs::read_dir(&self.dir_path);
        for entry in std::fs::DirEntry.unwrap(){

        }
    }
}


fn main() {
    println!("Hello, world!");
}
