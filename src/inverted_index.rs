
use std::collections::HashMap;

// TODO: port tests

use serde::ser::{Serialize, Serializer, SerializeMap};

#[derive(Debug, Copy, Clone, Serialize)]
struct TermFrequency {
    tf: f32,
}

#[derive(Debug, Clone)]
pub struct IndexItem {
    docs: HashMap<String, TermFrequency>,
    df: usize,
    children: HashMap<String, IndexItem>,
}

impl IndexItem {
    pub fn new() -> Self {
        IndexItem {
            docs: HashMap::new(),
            df: 0,
            children: HashMap::new(),
        }
    }

    fn update_children(&mut self, token: &str, doc_ref: &str, freq: f32) {
        let mut char_indices = token.char_indices();
        if let Some((_, char)) = char_indices.next() {
            let item = self.children.entry(char.to_string()).or_insert(
                IndexItem::new(),
            );
            if let Some((idx, _)) = char_indices.next() {
                item.update_children(&token[idx..], doc_ref, freq);
            }
        }
    }

    pub fn add_token(&mut self, token: &str, doc_ref: &str, freq: f32) {
        self.update_children(token, doc_ref, freq);

        if self.docs.contains_key(doc_ref) {
            self.df += 1;
        }
        self.docs.insert(doc_ref.into(), TermFrequency { tf: freq });
    }
}

// Manually implement serialize so `children` are inline
impl Serialize for IndexItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(2 + self.children.len()))?;
        state.serialize_entry("df", &self.df)?;
        state.serialize_entry("docs", &self.docs)?;

        for (key, value) in &self.children {
            state.serialize_entry(key, &value)?;
        }

        state.end()
    }
}

#[derive(Serialize, Debug)]
pub struct InvertedIndex {
    root: IndexItem,
}

impl InvertedIndex {
    pub fn new() -> Self {
        InvertedIndex { root: IndexItem::new() }
    }

    pub fn add_token(&mut self, token: &str, doc_ref: &str, freq: f32) {
        self.root.add_token(token, doc_ref, freq);
    }
}
