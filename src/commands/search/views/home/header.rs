use crate::commands::search::state::UIState;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::DOT;
use ratatui::text::Line;
use ratatui::widgets::{canvas, Block, BorderType, Borders, Padding, Tabs};
use ratatui::{text, Frame};

/// Create the tabs widget
fn create_tabs<'a>(tab_titles: Vec<String>, list_tab: usize) -> anyhow::Result<Tabs<'a>> {
    let items = tab_titles
        .into_iter()
        .map(|t| Line::from(format!(" {} ", t)))
        .collect::<Vec<Line>>();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Black));
    let tabs = Tabs::new(items.to_vec())
        .block(block)
        .style(Style::default().fg(Color::White).bold())
        .highlight_style(Style::default().black().on_light_yellow().bold())
        .select(list_tab)
        .divider(DOT);
    Ok(tabs)
}

/// Renders the home view header (collection tabs)
pub fn render_home_header(app: &mut UIState, f: &mut Frame, chunk: Rect) -> anyhow::Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(13),
        ])
        .split(chunk);

    let tabs = create_tabs(app.collection_tab_titles(), app.collection_tab)?;

    let logo = canvas::Canvas::default()
        .block(
            Block::default()
                .padding(Padding::new(0, 0, 0, 0))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().yellow()),
        )
        .x_bounds([0.0, 11.5])
        .y_bounds([0.0, 5.0])
        .paint(|ctx| {
            ctx.draw(&canvas::Line {
                x1: 1.5,
                x2: 3.0,
                y1: 0.0,
                y2: 0.0,
                color: Color::LightYellow,
            });
            ctx.layer();
            ctx.draw(&canvas::Line {
                x1: 7.0,
                x2: 10.0,
                y1: 2.5,
                y2: 2.5,
                color: Color::LightYellow,
            });
            ctx.layer();
            ctx.print(
                3.5,
                1.0,
                text::Line::from(text::Span::from("gitnr").bold().italic().light_yellow()),
            );
        });

    f.render_widget(tabs, chunks[0]);
    f.render_widget(logo, chunks[2]);
    Ok(())
}
