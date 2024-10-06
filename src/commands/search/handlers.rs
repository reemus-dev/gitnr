use crate::commands::search::framework::tui::Tui;
use crate::commands::search::state::view_preview::{UIStatePreview, UIStatePreviewState};
use crate::commands::search::state::{UIState, UIStateView};
use crate::template::list::TemplateList;
use anyhow::Result;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use std::io::Stderr;
use std::time::{Duration, Instant};
use tui_input::backend::crossterm::EventHandler;

/// Handles the keyboard events for the UI
pub fn handle_key_events(
    event: KeyEvent,
    app: &mut UIState,
    tui: &mut Tui<CrosstermBackend<Stderr>>,
) -> Result<()> {
    // Ignore release events to prevent executing the same action twice
    // [Issue] https://github.com/reemus-dev/gitnr/issues/3
    if event.kind == KeyEventKind::Release {
        return Ok(());
    }

    let is_shift = event.modifiers == KeyModifiers::SHIFT;
    let is_ctrl = event.modifiers == KeyModifiers::CONTROL;
    let is_c = event.code == KeyCode::Char('c') || event.code == KeyCode::Char('C');

    // ---------------
    // Application exit: Ctrl + C
    // ---------------
    let is_quit_ctrl_c = is_ctrl && is_c;
    if is_quit_ctrl_c {
        app.quit();
        return Ok(());
    }

    // ---------------
    // Handle shift modified keys presses
    // ---------------
    if is_shift {
        match &mut app.view {
            UIStateView::Home => match event.code {
                // Shift + S: previews the currently selected templates
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    // Clear the terminal before changing the view to prevent weird left overs
                    tui.clear()?;
                    let templates = app
                        .selected
                        .lock()
                        .unwrap()
                        .iter()
                        .map(|s| s.template.clone())
                        .collect::<Vec<_>>();
                    let templates = TemplateList::new(templates);
                    let preview_state = UIStatePreview::new(templates)?;
                    app.view = UIStateView::Preview(preview_state);
                    return Ok(());
                }
                // Shift + C: previews the currently highlighted template in the list
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    let index = app.collection().state.lock().unwrap().selected();
                    match index {
                        None => {}
                        Some(index) => {
                            let template = app.collection().values.get(index).cloned();
                            if let Some(template) = template {
                                // Clear the terminal before changing the view to prevent weird left overs
                                tui.clear()?;
                                let templates = TemplateList::new(vec![template]);
                                let preview_state = UIStatePreview::new(templates)?;
                                app.view = UIStateView::Preview(preview_state);
                            }
                            return Ok(());
                        }
                    }
                }
                _ => {}
            },
            UIStateView::Preview(ref mut p) => match event.code {
                KeyCode::Char('c') | KeyCode::Char('C') => p.copy_content()?,
                KeyCode::Char('x') | KeyCode::Char('X') => p.copy_command()?,
                _ => {}
            },
        }
    }

    // ---------------
    // View: Home
    // ---------------
    if let UIStateView::Home = &mut app.view {
        match event.code {
            // ---------------
            // Change tabs: left & right arrow
            // ---------------
            KeyCode::Right => {
                app.collection_next();
                app.list_filter_update();
            }
            KeyCode::Left => {
                app.collection_prev();
                app.list_filter_update();
            }
            // ---------------
            // Select template: Enter
            // ---------------
            KeyCode::Enter => app.list_select(),
            // ---------------
            // Change template
            //  - Up & Down Arrow  (next / previous)
            //  - Home & End       (top / bottom)
            // ---------------
            KeyCode::Up => {
                app.list_previous(if is_shift { Some(10) } else { Some(1) });
            }
            KeyCode::Down => {
                app.list_next(if is_shift { Some(10) } else { Some(1) });
            }
            KeyCode::Home => {
                let list = app.collection();
                list.state.lock().unwrap().select(Some(0));
            }
            KeyCode::End => {
                let list = app.collection();
                list.state
                    .lock()
                    .unwrap()
                    .select(Some(list.values.len().saturating_sub(1)));
            }
            // ---------------
            // Filtering collection
            //  - Space only if value is not empty
            //  - Any character/number
            // ---------------
            KeyCode::Char(' ') => {
                if !app.collection_filter.value().is_empty() {
                    app.collection_filter.handle_event(&Event::Key(event));
                    app.list_filter_update();
                }
            }
            _ => {
                app.collection_filter.handle_event(&Event::Key(event));
                app.list_filter_update();
            }
        }
    }

    // ---------------
    // View: Preview
    // ---------------
    if let UIStateView::Preview(ref mut p) = &mut app.view {
        match &p.state {
            UIStatePreviewState::Default => match event.code {
                // Return to home view with ESC
                KeyCode::Esc => {
                    app.view = UIStateView::Home;
                }
                // Scroll content up / down
                KeyCode::Up => p.scroll_up(),
                KeyCode::Down => p.scroll_down(),
                KeyCode::Home => p.scroll_to_top(),
                KeyCode::End => p.scroll_to_bottom(),
                _ => {}
            },
            // Allow closing success message with any key except the keys that triggered the copy action
            // Without this, the success message closes when the user presses the key that triggered the copy action
            UIStatePreviewState::CopiedContent => match event.code {
                KeyCode::Char('c') | KeyCode::Char('C') => {}
                _ => p.copy_done(),
            },
            UIStatePreviewState::CopiedCommand => match event.code {
                KeyCode::Char('x') | KeyCode::Char('X') => {}
                _ => p.copy_done(),
            },
        }
    }

    Ok(())
}

/// Handles mouse events for the UI
pub fn handle_mouse_events(event: MouseEvent, app: &mut UIState) -> Result<()> {
    let now = Instant::now();

    // For whatever reason these event modifiers aren't detected
    let is_alt = event.modifiers == KeyModifiers::ALT;
    let is_shift = event.modifiers == KeyModifiers::SHIFT;

    // ---------------
    // View: Home
    // ---------------
    if let UIStateView::Home = &app.view {
        match event.kind {
            // Select a template with left-click
            MouseEventKind::Down(MouseButton::Left) => app.list_select(),
            // Scrolling on the collection templates list
            MouseEventKind::ScrollUp => {
                if now.duration_since(app.last_scroll_time) > Duration::from_millis(15) {
                    app.list_previous(if is_shift || is_alt {
                        Some(10)
                    } else {
                        Some(1)
                    });
                    app.last_scroll_time = now;
                }
            }
            // Scrolling on the collection templates list
            MouseEventKind::ScrollDown => {
                if now.duration_since(app.last_scroll_time) > Duration::from_millis(15) {
                    app.list_next(if is_shift || is_alt {
                        println!("Scrolling {} {}", is_shift, is_alt);
                        Some(10)
                    } else {
                        Some(1)
                    });
                    app.last_scroll_time = now;
                }
            }
            _ => {}
        }
    }

    // ---------------
    // View: Preview
    // ---------------
    if let UIStateView::Preview(ref mut p) = &mut app.view {
        match &p.state {
            // Scrolling content up / down with mouse wheel
            UIStatePreviewState::Default => match event.kind {
                MouseEventKind::ScrollUp => p.scroll_up(),
                MouseEventKind::ScrollDown => p.scroll_down(),
                _ => {}
            },
            // Closing success message with mouse click
            _ => {
                if let MouseEventKind::Down(MouseButton::Left) = event.kind {
                    p.copy_done()
                }
            }
        }
    }

    Ok(())
}
