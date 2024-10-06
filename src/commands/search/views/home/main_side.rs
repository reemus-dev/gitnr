use crate::commands::search::state::collection::UICollectionSelection;
use crate::commands::search::state::UIState;
use crate::commands::search::views::util;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph};
use ratatui::Frame;
use std::sync::MutexGuard;

/// Create the filter input widget
fn create_filter_input(app: &mut UIState) -> anyhow::Result<Paragraph> {
    let input = &app.collection_filter;
    let is_filtering = app.list_is_filtering();
    let title = util::title_string("Filter".into());
    let mut block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1));
    if is_filtering {
        block = block.border_style(Style::default().light_yellow());
    }
    let paragraph = Paragraph::new(input.value())
        .style(Style::default())
        .block(block);
    Ok(paragraph)
}

/// Create the selected templates list widget
fn create_selected<'a>(
    selected: &MutexGuard<Vec<UICollectionSelection>>,
) -> anyhow::Result<List<'a>> {
    let items: Vec<ListItem> = selected
        .iter()
        .map(|s| {
            let name = s.template.value.name().unwrap();
            let prefix = s.kind.name();
            ListItem::new(format!("{} - {}", prefix, name))
        })
        .collect();

    let title = format!("Selection ({})", selected.len());
    let title = util::title_string(title);
    let title = Title::from(title);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1));

    let widget = List::new(items)
        .block(block)
        .style(Style::default().fg(Color::White));

    Ok(widget)
}

/// Render the right sidebar of the home view (filter input & selected templates)
pub fn render_home_main_side(app: &mut UIState, f: &mut Frame, chunk: Rect) -> anyhow::Result<()> {
    let chunks = Layout::default()
        .constraints(vec![Constraint::Length(3), Constraint::Min(0)])
        .split(chunk);

    let top = chunks[0];
    let bottom = chunks[1];

    // Create selected list
    let selected_widget = create_selected(&app.selected.lock().unwrap())?;

    // Create filter input widget
    let is_filtering = app.list_is_filtering();
    let filter_widget = create_filter_input(app)?;

    f.render_widget(filter_widget, top);
    f.render_widget(selected_widget, bottom);

    if is_filtering {
        f.set_cursor_position(Position::new(
            // Put cursor past the end of the input text
            top.x + app.collection_filter.visual_cursor() as u16 + 2,
            // Move one line down, from the border to the input line
            top.y + 1,
        ))
    }

    Ok(())
}
