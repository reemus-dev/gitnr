mod framework;
mod handlers;
mod state;
mod views;

use crate::commands::search::framework::event::{Event, EventHandler};
use crate::commands::search::framework::tui::Tui;
use crate::commands::search::handlers::{handle_key_events, handle_mouse_events};
use crate::commands::search::state::UIState;
use anyhow::Result;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

pub fn command() -> Result<()> {
    // Create an application state
    let mut app = UIState::new()?;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;

        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app, &mut tui)?,
            Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
