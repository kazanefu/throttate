use serde::{Deserialize, Serialize};
mod load;

pub use load::get_settings;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub window: WindowSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub fullscreen: bool,
    pub vsync: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "throtate".into(),
            fullscreen: false,
            vsync: true,
        }
    }
}
