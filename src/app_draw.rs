use std::io;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::AppState;

pub fn draw_app(app: &AppState, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
    let size = f.size();
    let view = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(size);

    let entry_tree_block = Block::default()
        .title("Entry Tree")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let items: Vec<ListItem> = app
        .entries()
        .iter()
        .enumerate()
        .map(|(_, entry)| {
            let filename = entry.file_name().unwrap().to_str().unwrap();
            let icon = match entry.is_dir() {
                true => "󰉋",
                false => "󰈚",
            };
            ListItem::new(format!("{icon} {filename}"))
        })
        .collect();

    let list = List::new(items).block(entry_tree_block).highlight_style(
        Style::default()
            .fg(Color::Blue)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    );

    // TODO: This isn't how it is done in the given examples
    let mut state = ListState::default();
    state.select(Some(app.selected));

    f.render_stateful_widget(list, view[0], &mut state);
}
