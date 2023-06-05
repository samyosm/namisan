use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{AppCommand, AppState};

pub fn update_app(app: &mut AppState, event: &KeyEvent) -> AppCommand {
    match event.code {
        KeyCode::Char('p') => {
            app.pwd = "Hello, world!";
        }
        KeyCode::Char('q') => return AppCommand::Quit,
        _ => {}
    }
    AppCommand::None
}
