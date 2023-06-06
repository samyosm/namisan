use std::path::Path;

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
        KeyCode::Char('q') => return AppCommand::Quit,
        _ => {}
    }
    AppCommand::None
}
