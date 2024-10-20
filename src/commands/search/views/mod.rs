mod home;
mod preview;
mod util;

use crate::commands::search::state::{UIState, UIStateView};
use ratatui::Frame;

/// Renders the user interface
pub fn render(app: &mut UIState, f: &mut Frame) -> anyhow::Result<()> {
    match &app.view {
        UIStateView::Home => home::render_home(app, f)?,
        UIStateView::Preview { .. } => preview::render_preview(app, f)?,
    }
    Ok(())
}
