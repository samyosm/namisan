mod app;
mod app_draw;
mod app_update;
mod metadata;

use app::{AppCommand, AppState};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, io, process::Command};

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

    let pwd = env::current_dir().expect("couldn't fetch current directory path");
    let mut app = AppState::new(pwd);
    /* Working */
    terminal.draw(|f| app.draw_app(f))?;

    // Note: this draws the UI only after recieving an input
    loop {
        let event = read_event();
        if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
            break;
        }

        match app.update_app(&event) {
            AppCommand::Quit => break,
            AppCommand::Clear => {
                terminal.flush().expect("couldn't clear the terminal");
                terminal.clear().expect("couldn't clear the terminal");
            }
            AppCommand::XDG(path) => {
                disable_raw_mode()?;
                execute!(
                    terminal.backend_mut(),
                    LeaveAlternateScreen,
                    DisableMouseCapture
                )?;
                terminal.show_cursor()?;

                Command::new("xdg-open")
                    .arg(path)
                    .status()
                    .expect("couldn't open file");

                terminal.flush().expect("couldn't clear the terminal");
                terminal.clear().expect("couldn't clear the terminal");

                let mut stdout = io::stdout();
                enable_raw_mode()?;
                execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
            }
            AppCommand::None => {}
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
