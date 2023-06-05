use std::io;

use crossterm::event::KeyEvent;
use tui::{backend::CrosstermBackend, Frame};

use crate::{app_draw, app_update};

pub struct AppState<'a> {
    pub pwd: &'a str,
    pub items: Vec<&'a str>,
    pub selected: usize,
}

impl AppState<'_> {
    pub fn draw_app(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
        app_draw::draw_app(&self, f);
    }

    pub fn update_app(&mut self, event: &KeyEvent) -> AppCommand {
        app_update::update_app(self, event)
    }
}

pub enum AppCommand {
    Quit,
    None,
}
