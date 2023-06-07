use std::{
    fs::{self, File, ReadDir},
    io::{self, Read},
    path::{Path, PathBuf},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Padding, Paragraph},
    Frame,
};

use crate::{app::AppState, metadata::Metadata};

pub fn draw_app(app: &AppState, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
    let size = f.size();

    let view = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(size);

    draw_header(app, f, view[0]);
    draw_body(app, f, view[1]);
}

fn draw_header(app: &AppState, f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
    let preview_block = Block::default()
        //TODO: Style the filename
        .title("Current working directory")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray))
        .border_type(BorderType::Rounded);

    let paragraph = Paragraph::new(app.pwd().to_str().unwrap()).block(preview_block);
    f.render_widget(paragraph, area)
}

fn draw_body(app: &AppState, f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
    let view = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(area);

    draw_entry_tree(app, f, view[0]);
    draw_entry_info(app, f, view[1]);
}

fn draw_entry_info(app: &AppState, f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(area);

    let entry = app.entries().get(app.selected).unwrap();
    draw_metadata(f, entry, layout[0]);
    draw_preview(f, entry, layout[1]);
}

fn draw_metadata(f: &mut Frame<CrosstermBackend<io::Stdout>>, entry: &PathBuf, area: Rect) {
    let metadata = Metadata::new(entry);

    let preview_block = Block::default()
        //TODO: Style the filename
        .title(format!("Metadata"))
        .padding(Padding::new(1, 1, 0, 0))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .border_type(BorderType::Rounded);

    let paragraph = Paragraph::new(metadata.display()).block(preview_block);
    f.render_widget(paragraph, area)
}

fn draw_preview(f: &mut Frame<CrosstermBackend<io::Stdout>>, entry: &PathBuf, area: Rect) {
    let preview_filename = entry.file_name().unwrap().to_str().unwrap();
    let preview_block = Block::default()
        //TODO: Style the filename
        .title(format!("Previewing {preview_filename}"))
        .padding(Padding::new(1, 1, 0, 0))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .border_type(BorderType::Rounded);

    if entry.is_dir() {
        let items = list_dir_entries(entry);

        let list = List::new(items).block(preview_block);

        f.render_widget(list, area);
    } else {
        let file = File::open(entry).expect("couldn't open entry");
        let mut vec_buf = Vec::new();
        file.take((area.bottom() * area.left()).into())
            .read_to_end(&mut vec_buf)
            .expect("couldn't read file content");
        let text = String::from_utf8_lossy(&vec_buf);
        let paragraph = Paragraph::new(text).block(preview_block);
        f.render_widget(paragraph, area)
    }
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

fn list_dir_entries(dir: &Path) -> Vec<ListItem> {
    let open_dir = fs::read_dir(dir).expect("coulnd't open directory");

    let items: Vec<ListItem> = open_dir
        .enumerate()
        .map(|(_, entry)| {
            let entry = entry.expect("couldn't get entry").path();
            let filename = entry.file_name().unwrap().to_str().unwrap();
            let icon = match entry.is_dir() {
                true => "󰉋",
                false => "󰈚",
            };
            ListItem::new(format!("{icon} {filename}"))
        })
        .collect();
    items
}
