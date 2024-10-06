use crate::template::list::TemplateList;
use anyhow::{anyhow, Result};
use copypasta::{ClipboardContext, ClipboardProvider};
use once_cell::sync::Lazy;
use ratatui::widgets::ScrollbarState;
use std::sync::Mutex;

static CLIPBOARD: Lazy<Mutex<ClipboardContext>> =
    Lazy::new(|| Mutex::new(ClipboardContext::new().unwrap()));

/// The current state of the preview view
#[derive(Debug, Clone)]
pub enum UIStatePreviewState {
    Default,
    /// Triggered when user copies the preview content to the clipboard
    CopiedContent,
    /// Triggered when user copies the CLI command to the clipboard
    CopiedCommand,
}

/// State of the preview view
#[derive(Debug, Clone)]
pub struct UIStatePreview {
    /// The current state of the preview view
    pub state: UIStatePreviewState,
    /// The state of the vertical scrollbar
    pub scroll_state: ScrollbarState,
    /// The current vertical scroll position
    pub scroll_pos: u16,
    /// The templates being previewed
    pub templates: TemplateList,
    /// Title of the preview view
    pub title: String,
    /// The display content to preview
    pub content: String,
    /// The number of lines in the display content
    pub content_lines: u16,
    /// The CLI command used to generate the preview content
    pub command: String,
}

impl UIStatePreview {
    /// Constructs a new instance of [`UIStatePreview`].
    pub fn new(templates: TemplateList) -> Result<Self> {
        let state = UIStatePreviewState::Default;

        let content = templates.content()?;
        let content_lines = (content.matches('\n').count() + 1) as u16;
        let command = templates.command()?;

        let title = if templates.len() == 1 {
            format!(" Preview: {} ", templates[0].value.name()?)
        } else {
            format!(" Preview: Selected ({}) ", templates.len())
        };

        let scroll_pos = 0;
        let scroll_state = ScrollbarState::default().content_length(content_lines.into());

        Ok(Self {
            state,
            scroll_state,
            scroll_pos,
            title,
            content,
            content_lines,
            command,
            templates,
        })
    }

    /// Scrolls to the top of the preview content
    pub fn scroll_to_top(&mut self) {
        self.scroll_pos = 0;
        self.scroll_state = self.scroll_state.position(0);
    }

    /// Scrolls to the bottom of the preview content
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_pos = self.content_lines.saturating_sub(1);
        self.scroll_state = self.scroll_state.position(self.scroll_pos.into());
    }

    /// Scrolls to the preview content up one increment
    pub fn scroll_up(&mut self) {
        self.scroll_pos = self.scroll_pos.saturating_sub(1);
        self.scroll_state = self.scroll_state.position(self.scroll_pos.into());
    }

    /// Scrolls to the preview content down one increment
    pub fn scroll_down(&mut self) {
        self.scroll_pos = self
            .scroll_pos
            .saturating_add(1)
            .clamp(0, self.content_lines.saturating_sub(1));
        self.scroll_state = self.scroll_state.position(self.scroll_pos.into());
    }

    /// Copies the preview content to the clipboard
    pub fn copy_content(&mut self) -> Result<()> {
        self.copy_to_clipboard(self.content.to_owned())?;
        self.state = UIStatePreviewState::CopiedContent;
        Ok(())
    }

    /// Copies the CLI command to the clipboard
    pub fn copy_command(&mut self) -> Result<()> {
        self.copy_to_clipboard(self.command.to_owned())?;
        self.state = UIStatePreviewState::CopiedCommand;
        Ok(())
    }

    /// Copies the given content to the clipboard
    fn copy_to_clipboard(&self, content: String) -> Result<()> {
        let mut clip = CLIPBOARD
            .lock()
            .map_err(|_| anyhow!("Failed to lock clipboard"))?;
        clip.set_contents(content)
            .map_err(|_| anyhow!("Failed to set clipboard content"))
    }

    /// Resets the preview state to default
    pub fn copy_done(&mut self) {
        self.state = UIStatePreviewState::Default;
    }
}
