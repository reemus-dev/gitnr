mod footer;
mod header;
mod main_list;
mod main_side;

use crate::commands::search::state::{UIState, UIStateView};
use crate::commands::search::views::home::footer::render_home_footer;
use crate::commands::search::views::home::header::render_home_header;
use crate::commands::search::views::home::main_list::render_home_main_list;
use crate::commands::search::views::home::main_side::render_home_main_side;
use anyhow::{bail, Result};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;

/// Renders the main home UI
pub fn render_home(app: &mut UIState, f: &mut Frame) -> Result<()> {
    match &mut app.view {
        UIStateView::Home => {
            // let app = Mutex::new(app);
            let layout = Layout::default()
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(0),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .vertical_margin(0)
                .horizontal_margin(1)
                .split(f.area());

            let main = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Min(40),
                    Constraint::Length(1),
                    Constraint::Length(40),
                ])
                .split(layout[2]);

            let header = layout[0];
            let main_left = main[0];
            let main_right = main[2];
            let footer = layout[3];

            render_home_header(app, f, header)?;
            render_home_main_list(app, f, main_left)?;
            render_home_main_side(app, f, main_right)?;
            render_home_footer(app, f, footer)?;
        }
        _ => bail!("Invalid UI State: attempting to render home when not currently in that view"),
    }
    Ok(())
}
