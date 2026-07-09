use std::fs::{self, DirEntry,Metadata};
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
        for entry in std::fs::read_dir(&self.dir_path).unwrap(){
            let dir: DirEntry = entry.unwrap();
            let meta: Metadata = dir.metadata().unwrap();

            let file_info: FileItem= FileItem { 
                name: dir.file_name().into_string().unwrap(), 
                path: dir.path(), 
                is_dir: meta.is_dir(),
                size: meta.len() };
            &self.dir_files.push(file_info);
        }
    }
}


fn main() {
    println!("Hello, world!");
}
