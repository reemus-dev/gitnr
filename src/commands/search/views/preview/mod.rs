mod footer;

use crate::commands::search::state::view_preview::UIStatePreviewState;
use crate::commands::search::state::{UIState, UIStateView};
use crate::commands::search::views::preview::footer::render_preview_footer;
use crate::commands::search::views::util::rect_center;
use anyhow::{bail, Result};
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{
    Block, BorderType, Borders, Clear, Padding, Paragraph, Scrollbar, ScrollbarOrientation, Wrap,
};
use ratatui::{text, Frame};

/// Renders the preview view
pub fn render_preview<B: Backend>(app: &mut UIState, f: &mut Frame<'_, B>) -> Result<()> {
    match &mut app.view {
        UIStateView::Preview(ref mut p) => {
            let size = f.size();
            let title = text::Span::from(p.title.as_str()).bold().white();

            let layout = Layout::default()
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .vertical_margin(0)
                .horizontal_margin(1)
                .split(size);

            let block = Block::default()
                .padding(Padding::new(2, 2, 1, 1))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().light_yellow());

            // The preview template content
            let mut content = Paragraph::new(p.content.as_str())
                .block(block.clone().title(title.clone()))
                .wrap(Wrap { trim: true })
                .scroll((p.scroll_pos, 0));

            // If user has copied the content or command, dim the background when showing success message
            match &p.state {
                UIStatePreviewState::Default => {}
                _ => {
                    let block = block
                        .title(title.dark_gray())
                        .border_style(Style::new().dark_gray());
                    content = content.block(block).dark_gray();
                }
            }

            let vertical_scrollbar = Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some(""))
                .end_symbol(Some(""))
                .track_symbol("-") // ─
                .thumb_symbol("░"); //

            f.render_widget(content, layout[1]);
            f.render_stateful_widget(vertical_scrollbar, layout[1], &mut p.scroll_state);
            render_preview_footer(f, layout[2])?;

            // Render the success popup if user copied content or command
            match &p.state {
                UIStatePreviewState::Default => {}
                UIStatePreviewState::CopiedContent | UIStatePreviewState::CopiedCommand => {
                    let popup_block = Block::default()
                        .title("─ Success ─")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::new().light_green())
                        .padding(Padding::new(2, 1, 0, 0));

                    let popup_note =
                        "Note: You may need to paste the copied content before exiting";

                    let popup_text = if let UIStatePreviewState::CopiedContent = &p.state {
                        Text::from(vec![
                            Line::from(text::Span::from("Template copied to clipboard")),
                            Line::from(text::Span::from(popup_note).italic().dark_gray()),
                        ])
                    } else {
                        Text::from(vec![
                            Line::from(text::Span::from("CLI command copied to clipboard")),
                            Line::from(text::Span::from(popup_note).italic().dark_gray()),
                            Line::from(""),
                            Line::from(
                                text::Span::from(format!(" {} ", p.command.as_str()))
                                    .bold()
                                    .black()
                                    .on_light_green(),
                            ),
                        ])
                    };

                    let popup_text_height = (popup_text.height() + 3) as u16;

                    let popup_content = Paragraph::new(popup_text)
                        .wrap(Wrap { trim: false })
                        .block(popup_block);

                    let area = if let UIStatePreviewState::CopiedContent = &p.state {
                        rect_center(72, popup_text_height - 1, None, size)
                    } else {
                        rect_center(72, popup_text_height, None, size)
                    };

                    f.render_widget(Clear, area); // this clears out the background
                    f.render_widget(popup_content, area);
                }
            }
        }
        _ => {
            bail!("Invalid UI State: attempting to render preview when not currently in that view")
        }
    }
    Ok(())
}
