use std::io;

use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Frame,
};

use crate::app::AppState;

pub fn draw_app(app: &AppState, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
    let size = f.size();
    let block = Block::default().title(app.pwd).borders(Borders::ALL);
    f.render_widget(block, size);
}
