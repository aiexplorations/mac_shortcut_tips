mod app;
mod models;
mod utils;

use app::MacShortcutTips;
use iced::{Application, Settings};

fn main() -> iced::Result {
    MacShortcutTips::run(Settings::default())
}
