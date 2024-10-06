use crate::commands::search::framework::event::EventHandler;
use crate::commands::search::state::UIState;
use crate::commands::search::views::render;
use anyhow::Result;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags,
};
use ratatui::crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::Terminal;
use std::io;
use std::panic;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;

        if cfg!(target_family = "unix") {
            // Keyboard event types are only reported on Windows by default, enable on Unix too
            ratatui::crossterm::execute!(
                io::stderr(),
                EnterAlternateScreen,
                EnableMouseCapture,
                PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
            )?;
        } else {
            // Don't add the keyboard enhancement flags on Windows, as it produces error:
            // - Keyboard progressive enhancement not implemented for the legacy Windows API.
            ratatui::crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
        }

        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.clear()?;
        Ok(())
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: tui::Terminal::draw
    /// [`rendering`]: crate::ui:render
    pub fn draw(&mut self, app: &mut UIState) -> Result<()> {
        self.terminal.draw(|frame| render(app, frame).unwrap())?;
        Ok(())
    }

    /// Clears the terminal screen.
    pub fn clear(&mut self) -> Result<()> {
        self.terminal.clear()?;
        Ok(())
    }
    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        if cfg!(target_family = "unix") {
            ratatui::crossterm::execute!(
                io::stdout(),
                LeaveAlternateScreen,
                DisableMouseCapture,
                PopKeyboardEnhancementFlags
            )?;
        } else {
            ratatui::crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        }
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
