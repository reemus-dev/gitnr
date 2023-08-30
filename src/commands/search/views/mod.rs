mod home;
mod preview;
mod util;

use crate::commands::search::state::{UIState, UIStateView};
use ratatui::backend::Backend;
use ratatui::Frame;

/// Renders the user interface
pub fn render<B: Backend>(app: &mut UIState, f: &mut Frame<'_, B>) -> anyhow::Result<()> {
    match &app.view {
        UIStateView::Home => home::render_home(app, f)?,
        UIStateView::Preview { .. } => preview::render_preview(app, f)?,
    }
    Ok(())
}
