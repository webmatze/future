mod app;
mod event;
mod ui;
mod widgets;
mod data;

use std::io;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use app::App;
use event::{Event, EventHandler};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and event handler
    let mut app = App::new();
    let event_handler = EventHandler::new(16); // 60 FPS (~16ms tick)

    // Main loop
    let result = run_app(&mut terminal, &mut app, &event_handler);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {err}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    event_handler: &EventHandler,
) -> io::Result<()> {
    while app.running {
        // Draw UI
        terminal.draw(|frame| ui::render(frame, app))?;

        // Handle events
        match event_handler.next()? {
            Event::Tick => {
                app.tick();
            }
            Event::Key(key_event) => {
                app.handle_key_event(key_event);
            }
            Event::Resize(width, height) => {
                app.handle_resize(width, height);
            }
            Event::Mouse(_) => {}
        }
    }

    Ok(())
}
