use crate::models::{Shortcut, ShortcutCategory};
use once_cell::sync::Lazy;

static SHORTCUTS: Lazy<Vec<Shortcut>> = Lazy::new(|| {
    vec![
        // Document shortcuts
        Shortcut {
            name: "Save".to_string(),
            key_combination: "⌘S".to_string(),
            description: "Save the current document".to_string(),
            category: "Document".to_string(),
        },
        Shortcut {
            name: "Save As".to_string(),
            key_combination: "⌘⇧S".to_string(),
            description: "Save the current document with a different name".to_string(),
            category: "Document".to_string(),
        },
        Shortcut {
            name: "Print".to_string(),
            key_combination: "⌘P".to_string(),
            description: "Print the current document".to_string(),
            category: "Document".to_string(),
        },
        // Edit shortcuts
        Shortcut {
            name: "Cut".to_string(),
            key_combination: "⌘X".to_string(),
            description: "Remove the selection and copy it to the Clipboard".to_string(),
            category: "Edit".to_string(),
        },
        Shortcut {
            name: "Copy".to_string(),
            key_combination: "⌘C".to_string(),
            description: "Copy the selection to the Clipboard".to_string(),
            category: "Edit".to_string(),
        },
        Shortcut {
            name: "Paste".to_string(),
            key_combination: "⌘V".to_string(),
            description: "Paste the Clipboard contents".to_string(),
            category: "Edit".to_string(),
        },
        Shortcut {
            name: "Undo".to_string(),
            key_combination: "⌘Z".to_string(),
            description: "Undo the last action".to_string(),
            category: "Edit".to_string(),
        },
        Shortcut {
            name: "Redo".to_string(),
            key_combination: "⌘⇧Z".to_string(),
            description: "Redo the last action".to_string(),
            category: "Edit".to_string(),
        },
        Shortcut {
            name: "Select All".to_string(),
            key_combination: "⌘A".to_string(),
            description: "Select all items".to_string(),
            category: "Edit".to_string(),
        },
        // View shortcuts
        Shortcut {
            name: "Show/Hide Sidebar".to_string(),
            key_combination: "⌘⌥S".to_string(),
            description: "Show or hide the sidebar".to_string(),
            category: "View".to_string(),
        },
        Shortcut {
            name: "Enter Full Screen".to_string(),
            key_combination: "⌃⌘F".to_string(),
            description: "Use the app in full screen".to_string(),
            category: "View".to_string(),
        },
        // Window shortcuts
        Shortcut {
            name: "Minimize".to_string(),
            key_combination: "⌘M".to_string(),
            description: "Minimize the window".to_string(),
            category: "Window".to_string(),
        },
        Shortcut {
            name: "Close Window".to_string(),
            key_combination: "⌘W".to_string(),
            description: "Close the current window".to_string(),
            category: "Window".to_string(),
        },
        Shortcut {
            name: "New Window".to_string(),
            key_combination: "⌘N".to_string(),
            description: "Open a new window".to_string(),
            category: "Window".to_string(),
        },
        // Finder shortcuts
        Shortcut {
            name: "New Finder Window".to_string(),
            key_combination: "⌘N".to_string(),
            description: "Open a new Finder window".to_string(),
            category: "Finder".to_string(),
        },
        Shortcut {
            name: "New Folder".to_string(),
            key_combination: "⌘⇧N".to_string(),
            description: "Create a new folder".to_string(),
            category: "Finder".to_string(),
        },
        Shortcut {
            name: "Search".to_string(),
            key_combination: "⌘F".to_string(),
            description: "Search for files and folders".to_string(),
            category: "Finder".to_string(),
        },
        // App shortcuts
        Shortcut {
            name: "Quit App".to_string(),
            key_combination: "⌘Q".to_string(),
            description: "Quit the current app".to_string(),
            category: "App".to_string(),
        },
        Shortcut {
            name: "Force Quit".to_string(),
            key_combination: "⌘⌥⎋".to_string(),
            description: "Choose an app to force quit".to_string(),
            category: "App".to_string(),
        },
        Shortcut {
            name: "Switch Apps".to_string(),
            key_combination: "⌘⇥".to_string(),
            description: "Switch to the next most recently used app".to_string(),
            category: "App".to_string(),
        },
        // Screenshot shortcuts
        Shortcut {
            name: "Full Screenshot".to_string(),
            key_combination: "⌘⇧3".to_string(),
            description: "Take a screenshot of the entire screen".to_string(),
            category: "Screenshots".to_string(),
        },
        Shortcut {
            name: "Selected Screenshot".to_string(),
            key_combination: "⌘⇧4".to_string(),
            description: "Take a screenshot of a selected portion".to_string(),
            category: "Screenshots".to_string(),
        },
        Shortcut {
            name: "Window Screenshot".to_string(),
            key_combination: "⌘⇧4 Space".to_string(),
            description: "Take a screenshot of a window".to_string(),
            category: "Screenshots".to_string(),
        },
        // System shortcuts
        Shortcut {
            name: "Lock Screen".to_string(),
            key_combination: "⌘⌃Q".to_string(),
            description: "Lock your screen".to_string(),
            category: "System".to_string(),
        },
        Shortcut {
            name: "Sleep".to_string(),
            key_combination: "⌥⌘⏏".to_string(),
            description: "Put your Mac to sleep".to_string(),
            category: "System".to_string(),
        },
        Shortcut {
            name: "System Settings".to_string(),
            key_combination: "⌘,".to_string(),
            description: "Open system settings".to_string(),
            category: "System".to_string(),
        },
    ]
});

pub fn load_shortcuts() -> Vec<ShortcutCategory> {
    let mut categories = vec![
        ShortcutCategory::new("Document".to_string()),
        ShortcutCategory::new("Edit".to_string()),
        ShortcutCategory::new("View".to_string()),
        ShortcutCategory::new("Window".to_string()),
        ShortcutCategory::new("Finder".to_string()),
        ShortcutCategory::new("App".to_string()),
        ShortcutCategory::new("Screenshots".to_string()),
        ShortcutCategory::new("System".to_string()),
    ];

    for shortcut in SHORTCUTS.iter() {
        if let Some(category) = categories.iter_mut().find(|c| c.name == shortcut.category) {
            category.add_shortcut(shortcut.clone());
        }
    }

    categories
}
