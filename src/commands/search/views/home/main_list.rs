use crate::commands::search::state::collection::UICollectionSelection;
use crate::commands::search::state::UIState;
use crate::commands::search::views::util;
use crate::template::item::Template;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Padding};
use ratatui::Frame;
use std::sync::MutexGuard;

/// Create the templates list widget
fn create_list<'a>(
    values: &[Template],
    state: &mut MutexGuard<ListState>,
    selected: &MutexGuard<Vec<UICollectionSelection>>,
) -> anyhow::Result<List<'a>> {
    let index = state.selected().unwrap_or(0) + 1;

    let items: Vec<ListItem> = values
        .iter()
        .map(|tmpl| {
            let name = tmpl.value.name().unwrap();
            let item = ListItem::new(name);
            match selected.iter().position(|s| &s.template == tmpl) {
                Some(_) => item.style(Style::default().bold().on_dark_gray()),
                None => item,
            }
        })
        .collect();

    let title = format!("List ({}/{})", index, values.len());
    let title = util::title_string(title);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::new(1, 1, 0, 0));

    let widget = List::new(items)
        .block(block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("▷ ");
    Ok(widget)
}

/// Renders the main collection templates list
pub fn render_home_main_list(app: &mut UIState, f: &mut Frame, chunk: Rect) -> anyhow::Result<()> {
    let list = app.collection();
    let state = &mut list.state.lock().unwrap();
    let widget = create_list(&list.values, state, &app.selected.lock().unwrap())?;
    f.render_stateful_widget(widget, chunk, state);
    Ok(())
}
