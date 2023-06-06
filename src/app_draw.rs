use std::{
    fs::{self, File},
    io::{self, Read},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Widget},
    Frame,
};

use crate::app::AppState;

pub fn draw_app(app: &AppState, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
    let size = f.size();
    let view = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(size);

    draw_entry_tree(app, f, view[0]);
    draw_preview(app, f, view[1]);
}

fn draw_preview(app: &AppState, f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
    let preview_entry = app.entries().get(app.selected).unwrap();
    let preview_filename = preview_entry.file_name().unwrap().to_str().unwrap();

    let preview_block = Block::default()
        //TODO: Style the filename
        .title(format!("Previewing {preview_filename}"))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let mut buffer = String::new();

    if preview_entry.is_dir() {
        // TODO
    } else {
        let mut file = File::open(preview_entry).expect("couldn't open entry");
        file.read_to_string(&mut buffer)
            .expect("couldn't read file to string");
    }
    let paragraph = Paragraph::new(buffer).block(preview_block);

    f.render_widget(paragraph, area)
}
fn draw_entry_tree(app: &AppState, f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
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

    f.render_stateful_widget(list, area, &mut state);
}
