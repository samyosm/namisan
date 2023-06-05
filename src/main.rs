use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Frame, Terminal,
};

struct AppState<'a> {
    pwd: &'a str,
    items: Vec<&'a str>,
    selected: usize,
}

impl AppState<'_> {
    fn draw_app(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let size = f.size();
        let block = Block::default().title(self.pwd).borders(Borders::ALL);
        f.render_widget(block, size);
    }

    fn update_app(&mut self, event: &KeyEvent) -> Option<AppCommand> {
        match event.code {
            KeyCode::Char('p') => {
                self.pwd = "Hello, world!";
            }
            KeyCode::Char('q') => return Some(AppCommand::Quit),
            _ => {}
        }
        None
    }
}

enum AppCommand {
    Quit,
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

    terminal.draw(|f| app.draw_app(f))?;

    loop {
        let event = read_event();
        if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
            break;
        }

        if let Some(command) = app.update_app(&event) {
            match command {
                AppCommand::Quit => break,
            }
        }

        terminal.draw(|f| app.draw_app(f))?;
    }

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
