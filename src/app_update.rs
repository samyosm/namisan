use std::{path::Path, process::Command};

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{AppCommand, AppState};

pub fn update_app(app: &mut AppState, event: &KeyEvent) -> AppCommand {
    match event.code {
        KeyCode::Char('p') => {
            let path = app.pwd().parent().unwrap_or(Path::new("/"));
            app.change_pwd(path.to_path_buf());
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected > 0 {
                app.selected -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.selected < app.entries().len() - 1 {
                app.selected += 1;
            }
        }
        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
            let new_path = app.entries().get(app.selected).unwrap();
            if new_path.is_dir() {
                app.change_pwd(new_path.to_path_buf());
            } else {
                return AppCommand::XDG(new_path.to_str().unwrap().to_string());
            }
        }
        KeyCode::Backspace | KeyCode::Left | KeyCode::Char('h') => {
            app.change_pwd(app.pwd().parent().unwrap().to_path_buf());
        }
        KeyCode::Char('q') => return AppCommand::Quit,
        _ => {}
    }
    AppCommand::None
}
