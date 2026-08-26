use std::fs::{self, DirEntry,Metadata};
use std::path::{self, PathBuf};
use std::env::{self, current_dir};
use crossterm::event::{Event,KeyCode};
use ratatui::widgets::{List, ListItem};
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
            self.dir_files.push(file_info);
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &AppState) {
    let items: Vec<_> = app.dir_files.iter().map(  
        | FileItem { name, path, is_dir, size } | 
        ListItem::new(name.clone())
    ).collect();
    let list = ratatui::widgets::List::new(items);
    
    f.render_widget(list, f.area());
}

//finally got this working 

fn main() {
    let mut app = AppState::new_state();
    app.scan_current_dir();

    crossterm::terminal::enable_raw_mode().unwrap();
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();

    let mut terminal = ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout())).unwrap();

    loop{
        terminal.draw(|f| {
            ui(f, &app);
        }).unwrap();

        let event = crossterm::event::read().unwrap();

        match event{

            Event::Key(key_event) =>{

                match key_event.code{  
                    KeyCode::Char('q') => {
                        break;
                    } KeyCode::Up => {
                        if app.cursor > 0{
                           app.cursor -= 1; 
                        } 
                    } KeyCode::Down => {
                        if !app.dir_files.is_empty() && app.cursor < app.dir_files.len() - 1{
                            app.cursor += 1;
                        }
                    } _ => {}

                }

            } _ => {

            }
        }
        
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();

}
