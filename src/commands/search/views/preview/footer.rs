use indoc::formatdoc;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui::{text, Frame};

/// Renders the preview footer (help section)
pub fn render_preview_footer<B: Backend>(f: &mut Frame<'_, B>, chunk: Rect) -> anyhow::Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(20),
            Constraint::Min(20),
            Constraint::Min(50),
        ])
        .margin(0)
        .horizontal_margin(1)
        .split(chunk);

    let text1 = formatdoc! {"
        Back: Esc
        Quit: Ctrl + C
    "};

    let text2 = formatdoc! {"
        Keyboard: 🠝 🠟
        Mouse:    Wheel
    "};

    let text3 = formatdoc! {"
        Copy Template: Shift + C
        Copy Command:  Shift + X
    "};

    let block = Block::default()
        .padding(Padding::new(0, 1, 0, 0))
        .dark_gray();

    let t1 = text::Span::from("App").bold().underlined();
    let p1 = Paragraph::new(text1)
        .style(Style::default())
        .block(block.clone().title(t1));

    let t2 = text::Span::from("Scrolling").bold().underlined();
    let p2 = Paragraph::new(text2)
        .style(Style::default())
        .block(block.clone().title(t2));

    let t3 = text::Span::from("Output").bold().underlined();
    let p3 = Paragraph::new(text3)
        .style(Style::default())
        .block(block.clone().title(t3));

    f.render_widget(p1, chunks[0]);
    f.render_widget(p2, chunks[1]);
    f.render_widget(p3, chunks[2]);

    Ok(())
}
