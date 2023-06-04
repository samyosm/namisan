use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Frame, Terminal,
};

struct AppState<'a> {
    pwd: &'a str,
    items: Vec<&'a str>,
    selected: usize,
}

fn draw_app(f: &mut Frame<CrosstermBackend<io::Stdout>>, app: &AppState) {
    let size = f.size();
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, size);
}

fn main() -> Result<(), io::Error> {
    /* Initializing app */
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // TODO: implement (#2)
    let mut app = AppState {
        pwd: "/home/samy/",
        items: vec!["/home/samy/libs/", "/home/samy/images/"],
        selected: 0,
    };
    /* Working */

    terminal.draw(|f| draw_app(f, &app))?;

    thread::sleep(Duration::from_millis(5000));

    /* Preparing to quit */
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
