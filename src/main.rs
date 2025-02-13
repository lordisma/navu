use std::io;

use ratatui::{backend::CrosstermBackend, Terminal};
use crate::{
    app::{App, AppResult},
    event::EventHandler,
    tui::Tui,
    error::Error
};

pub mod app;
pub mod event;
pub mod tui;
pub mod ui;
pub mod error;
pub mod search;
pub mod table;

pub(crate) mod fuzzy;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        let event = tui
            .events
            .next()
            .await
            .map_err(|_| Error::Custom(String::from("failed to get the next event")))?;

        app.handle(event);
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
