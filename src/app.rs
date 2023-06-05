use std::{
    env, io,
    path::{Path, PathBuf},
};

use crossterm::event::KeyEvent;
use tui::{backend::CrosstermBackend, Frame};

use crate::{app_draw, app_update};

pub struct AppState {
    pwd: PathBuf,
    entries: Vec<PathBuf>,
    selected: usize,
}

impl AppState {
    fn fetch_entries(path: &PathBuf) -> Vec<PathBuf> {
        path.read_dir()
            .expect("couldn't read directory")
            .map(|entry| {
                let entry = entry.expect("couldn't fetch entry");
                entry.path()
            })
            .collect()
    }

    pub fn new(pwd: PathBuf) -> Self {
        let items = Self::fetch_entries(&pwd);
        Self {
            pwd,
            entries: items,
            selected: 0,
        }
    }

    pub fn change_pwd(&mut self, pwd: PathBuf) {
        *self = Self::new(pwd);
    }

    pub fn draw_app(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
        app_draw::draw_app(&self, f);
    }

    pub fn update_app(&mut self, event: &KeyEvent) -> AppCommand {
        app_update::update_app(self, event)
    }

    /* Getters */
    pub fn pwd(&self) -> &PathBuf {
        &self.pwd
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn entries(&self) -> &Vec<PathBuf> {
        &self.entries
    }
}

pub enum AppCommand {
    Quit,
    None,
}
