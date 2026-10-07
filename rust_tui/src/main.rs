use std::fs::{self, DirEntry,Metadata};
use std::path::{self, Ancestors, Path, PathBuf};
use std::env::{self, current_dir};
use std::process::Stdio;
use crossterm::event::{Event,KeyCode};
use crossterm::execute;
use ratatui::style::Color::{self, Yellow};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph};
use::ratatui::layout::{Constraint, Layout};

use crate::ActivePane::{FileList, Preview};

pub enum ActivePane {
    FileList,
    Preview,
}

pub struct AppState{
    //Dir = Directory
    pub dir_path: std::path::PathBuf, 
    pub dir_files: Vec<FileItem>,
    pub cursor : usize,
    pub preview_text : Option<String>,
    pub active_pane : ActivePane,
    pub scroller: u16, 
}

struct FileItem{
    name: String,
    path: std::path::PathBuf,
    is_dir: bool,
    size: u64
}

struct TerminalGuardRail;

impl AppState{
    pub fn new_state() -> Self{
        AppState{
            dir_path: std::env::current_dir().unwrap(),
            dir_files: Vec::new(),
            cursor: 0, 
            preview_text : None,
            active_pane : ActivePane::FileList,
            scroller: 0,
        }
    }

    pub fn scan_current_dir(&mut self){

        // Basically everytime we render a new page of the directory were in we need to clear the old one then load the new one
        self.dir_files.clear();

        // This is just gathering the content 
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


impl TerminalGuardRail{
    // Ensuring we do this operaion correctly for file editing access
    pub fn new() -> Self{
        crossterm::terminal::disable_raw_mode().unwrap();
        crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        return Self;
    }
}

impl Drop for TerminalGuardRail {
    fn drop(&mut self) {
        crossterm::terminal::enable_raw_mode().unwrap();
        crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
    }
}

fn ui(f: &mut ratatui::Frame, app: &AppState) {

    let items: Vec<_> = app.dir_files.iter().map(  
        | FileItem { name, path, is_dir, size } | 
        ListItem::new(name.clone())
    ).collect();

    let list = ratatui::widgets::List::new(items).highlight_symbol(">> ");
    let mut state = ratatui::widgets::ListState::default();

    let horizontal = Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]);
    let [list_area, preview_area] = horizontal.areas(f.area());

    let text_to_display = if let Some(content) = &app.preview_text {
        content
    } else {
        "Press Enter to preview a file"
    };

    let paragraph = Paragraph::new(text_to_display).block(Block::bordered().title("Preview"));
    state.select(Some(app.cursor));

    let _active_widget = Paragraph::new(text_to_display)
    .block(Block::bordered().border_style(Color::Yellow))
    .scroll((app.scroller,0));

    f.render_stateful_widget(list, list_area, &mut state);
    f.render_widget(paragraph, preview_area);
}


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
                        match app.active_pane{
                            ActivePane::FileList => {
                                if app.cursor > 0{
                                    app.cursor -= 1; 
                                } 

                            } ActivePane::Preview => {
                               if app.scroller > 0 { 
                                    app.scroller -= 1;
                               }
                            }
                        }
                    } KeyCode::Down => {
                        match app.active_pane{
                            ActivePane::FileList => {
                                if !app.dir_files.is_empty() && app.cursor < app.dir_files.len() - 1{
                                    app.cursor += 1;
                                }
                            } ActivePane::Preview => {
                                app.scroller += 1;
                            }
                        }
                    } KeyCode::Enter => {

                        if app.dir_files[app.cursor].is_dir {
                            app.dir_path = app.dir_files[app.cursor].path.clone();
                            app.scan_current_dir(); // Reset the file 
                            app.cursor = 0; 
                        } else {
                                //after a bunch of revision its just better to make this its own variable
                                let file_path = app.dir_files[app.cursor].path.clone();

                                //Suspends the current Tui
                                let _guard = TerminalGuardRail::new();
                                let mut _text_editor = std::process::Command::new("nano")
                                .arg(&file_path)
                                .status()
                                .unwrap();

                                //Resumes it
                                drop(_guard);

                                terminal.clear().unwrap();

                            if let Ok(contents) = fs::read_to_string(&file_path){
                                app.preview_text = Some(contents);
                            }
                        }
                    } KeyCode::Backspace => {
                        if let Some(parent_folder) = app.dir_path.parent(){
                            app.dir_path = (&parent_folder).to_path_buf();
                            app.scan_current_dir();
                            app.cursor = 0; 
                        }

                    } KeyCode::Tab => {

                    } _ => {}

                }

            } _ => {

            }
        }
        
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();

}
