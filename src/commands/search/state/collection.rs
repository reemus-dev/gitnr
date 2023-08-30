use crate::template::collection::{TemplateCollection, TemplateCollectionKind};
use crate::template::item::Template;
use anyhow::Result;
use ratatui::widgets::ListState;
use std::sync::{Arc, Mutex};

/// Stores the data and state for a template collection list
#[derive(Debug)]
pub struct UICollection {
    /// The underlying template collection
    pub collection: TemplateCollection,
    /// The state of the list
    pub state: Arc<Mutex<ListState>>,
    /// The current values of the list to display depending on any filters
    pub values: Vec<Template>,
}

impl UICollection {
    /// Constructs a new instance of [`UICollection`].
    pub fn new(kind: TemplateCollectionKind) -> Result<UICollection> {
        let collection = TemplateCollection::new(kind)?;

        // Default values to all the templates in the collection
        let values = collection.items.clone();

        // Create the list state and set the first item as selected
        let mut state = ListState::default();
        state.select(Some(0));

        let state = Arc::new(Mutex::new(state));

        Ok(UICollection {
            collection,
            values,
            state,
        })
    }

    /// Selects the next item in the list
    pub fn next(&self, inc: Option<usize>) {
        let length = self.values.len();
        if length == 0 {
            return;
        }
        let inc = inc.unwrap_or(1).min(length);
        let mut state = self.state.lock().unwrap();
        let i = state.selected().map_or(0, |i| (i + inc) % length);
        state.select(Some(i));
    }

    /// Selects the previous item in the list
    pub fn previous(&self, inc: Option<usize>) {
        let length = self.values.len();
        if length == 0 {
            return;
        }
        let inc = inc.unwrap_or(1).min(length);
        let mut state = self.state.lock().unwrap();
        let i = state.selected().map_or(0, |i| (i + length - inc) % length);
        state.select(Some(i));
    }
}

/// Represents an select item from the template collection list
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UICollectionSelection {
    pub kind: TemplateCollectionKind,
    pub template: Template,
}
