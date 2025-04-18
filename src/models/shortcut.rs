use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub name: String,
    pub key_combination: String,
    pub description: String,
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct ShortcutCategory {
    pub name: String,
    pub shortcuts: Vec<Shortcut>,
}

impl ShortcutCategory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            shortcuts: Vec::new(),
        }
    }

    pub fn add_shortcut(&mut self, shortcut: Shortcut) {
        self.shortcuts.push(shortcut);
    }
}
