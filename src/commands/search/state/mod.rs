/// Stateful list widget
pub mod collection;

/// Application preview state
pub mod view_preview;

use crate::commands::search::state::collection::{UICollection, UICollectionSelection};
use crate::commands::search::state::view_preview::UIStatePreview;
use crate::template::collection::TemplateCollectionKind;
use anyhow::Result;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;
use tui_input::Input;

/// Represents the current view of the terminal UI
#[derive(Debug, Clone)]
pub enum UIStateView {
    /// Home screen to view, filter and select from template collections
    Home,
    /// Preview screen to view the output for selected templates
    Preview(UIStatePreview),
}

/// Application state
#[derive(Debug)]
pub struct UIState {
    /// Is the application running?
    pub running: bool,
    /// The current view of the terminal UI
    pub view: UIStateView,
    /// Selected templates
    pub selected: Arc<Mutex<Vec<UICollectionSelection>>>,
    /// Index of the template collection tabs
    pub collection_tab: usize,
    /// Filter input for filtering the current template collection tab
    pub collection_filter: Input,
    /// List of available template collections
    pub collections: Vec<Arc<Mutex<UICollection>>>,
    /// The last time the user has scrolled the terminal (for debouncing scroll events)
    pub last_scroll_time: Instant,
}

impl UIState {
    /// Constructs a new instance of [`UIState`].
    pub fn new() -> Result<Self> {
        let collection = |kind: TemplateCollectionKind| -> Result<Arc<Mutex<UICollection>>> {
            let collection = UICollection::new(kind)?;
            Ok(Arc::new(Mutex::new(collection)))
        };

        Ok(Self {
            running: true,
            view: UIStateView::Home,
            selected: Arc::new(Mutex::new(vec![])),
            collection_tab: 0,
            collection_filter: Input::default(),
            collections: vec![
                collection(TemplateCollectionKind::TopTal)?,
                collection(TemplateCollectionKind::GitHub)?,
                collection(TemplateCollectionKind::GitHubGlobal)?,
                collection(TemplateCollectionKind::GitHubCommunity)?,
            ],
            last_scroll_time: Instant::now(),
        })
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Returns a list of tab titles for the template collections
    pub fn collection_tab_titles(&self) -> Vec<String> {
        self.collections
            .iter()
            .map(|list| list.lock().unwrap().collection.kind.name().to_string())
            .collect()
    }

    /// Goes to the next template collection tab on the right
    pub fn collection_next(&mut self) {
        self.collection_tab = (self.collection_tab + 1) % self.collections.len();
    }

    /// Goes to the previous template collection tab on the left
    pub fn collection_prev(&mut self) {
        self.collection_tab =
            (self.collection_tab + self.collections.len() - 1) % self.collections.len();
    }

    /// Returns the current tab template collection
    pub fn collection(&self) -> MutexGuard<'_, UICollection> {
        self.collections[self.collection_tab].lock().unwrap()
    }

    /// Moves to the next item in the template collection list
    pub fn list_next(&self, inc: Option<usize>) {
        let list = self.collection();
        list.next(inc);
    }

    /// Moves to the previous item in the template collection list
    pub fn list_previous(&self, inc: Option<usize>) {
        let list = self.collection();
        list.previous(inc);
    }

    /// Selects an item in the template collection list
    pub fn list_select(&self) {
        let list = self.collection();
        let state = list.state.lock().unwrap();
        let index = state.selected().unwrap_or(0);
        let template = list.values.get(index);
        if let Some(template) = template {
            let selection = UICollectionSelection {
                kind: list.collection.kind.clone(),
                template: template.clone(),
            };

            let mut selected = self.selected.lock().unwrap();
            let index = selected.iter().position(|s| s == &selection);
            match index {
                Some(index) => {
                    selected.remove(index);
                }
                None => {
                    selected.push(selection);
                }
            }
        }
    }

    /// Filters item in the template collection list based on the filter input
    pub fn list_filter_update(&self) {
        let mut list = self.collection();
        let value = self.collection_filter.value();

        list.values = if value.is_empty() {
            list.collection.items.clone()
        } else {
            let value = value.to_lowercase();
            list.collection
                .items
                .iter()
                .filter(|v| v.value.name().unwrap().to_lowercase().contains(&value))
                .cloned()
                .collect()
        }
    }

    /// Returns true if the template collection list is filtering item
    pub fn list_is_filtering(&self) -> bool {
        let value = self.collection_filter.value();
        !value.is_empty()
    }
}
