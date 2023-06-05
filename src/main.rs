use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
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

fn read_event() -> KeyEvent {
    loop {
        if let Ok(Event::Key(event)) = event::read() {
            return event;
        }
    }
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

    // TODO:
    // 1. Create read_event loop like during test
    // 2. Send events to `update_app`
    // 3. Create an outer loop with `draw_app` and `update_app`

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
