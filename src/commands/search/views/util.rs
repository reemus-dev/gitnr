use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Helper function for creating a UI block title that extends the border lines
pub fn title_string(title: String) -> String {
    // format!("{}  {} {}", "⎯", title, "⎯⎯")
    format!("{} {} {}", "", title, "")
}

/// Helper function to create a centered rect using up certain percentage of the available rect `r`
/// Taken from: https://github.com/ratatui-org/ratatui/blob/dc552116cf5e83c7ffcc2f5299c00d2315490c1d/examples/popup.rs#L96
pub fn rect_center_pct(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn rect_center(width: u16, height: u16, y_offset: Option<i16>, r: Rect) -> Rect {
    let height_buffer: i16 = ((r.height - height) / 2) as i16;
    let width_buffer: i16 = ((r.width - width) / 2) as i16;
    let y_offset = y_offset.unwrap_or(0);

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(height_buffer.saturating_add(y_offset) as u16),
                Constraint::Length(height),
                Constraint::Length(height_buffer.saturating_sub(y_offset) as u16),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(width_buffer as u16),
                Constraint::Length(width),
                Constraint::Length(width_buffer as u16),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
