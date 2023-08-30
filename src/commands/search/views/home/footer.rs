use crate::commands::search::state::UIState;
use indoc::formatdoc;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui::{text, Frame};

/// Renders the home UI footer (help section)
pub fn render_home_footer<B: Backend>(
    _app: &mut UIState,
    f: &mut Frame<'_, B>,
    chunk: Rect,
) -> anyhow::Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),
            Constraint::Length(38),
            Constraint::Length(26),
            Constraint::Length(26),
        ])
        .margin(0)
        .horizontal_margin(1)
        .split(chunk);

    let text1 = formatdoc! {"
        Quit: Ctrl + C
    "};

    let text2 = formatdoc! {"
        Tabs: 🠜 🠞
        List: 🠝 🠟 or M. Wheel (+Shift=fast)
    "};

    let text3 = formatdoc! {"
        Select: Enter
        Filter: Start typing
    "};

    let text4 = formatdoc! {"
        Current:   Shift + C
        Selection: Shift + S
    "};

    let block = Block::default()
        .padding(Padding::new(0, 1, 0, 0))
        .dark_gray();

    let t1 = text::Span::from("App").bold().underlined(); // ⎯
    let p1 = Paragraph::new(text1)
        .style(Style::default())
        .block(block.clone().title(t1));

    let t2 = text::Span::from("Templates").bold().underlined();
    let p2 = Paragraph::new(text2)
        .style(Style::default())
        .block(block.clone().title(t2));

    let t3 = text::Span::from("").bold().underlined();
    let p3 = Paragraph::new(text3)
        .style(Style::default())
        .block(block.clone().title(t3));

    let t4 = text::Span::from("Preview & Generate").bold().underlined();
    let p4 = Paragraph::new(text4)
        .style(Style::default())
        .block(block.clone().title(t4));

    f.render_widget(p1, chunks[0]);
    f.render_widget(p2, chunks[1]);
    f.render_widget(p3, chunks[2]);
    f.render_widget(p4, chunks[3]);

    Ok(())
}
