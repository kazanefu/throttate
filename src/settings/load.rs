use super::Settings;
use std::fs;

use anyhow::Error;

fn load_settings() -> Result<Settings, Error> {
    let settings_str = fs::read_to_string("settings/settings.toml")?;
    let settings: Settings = toml::from_str(&settings_str)?;
    Ok(settings)
}

pub fn get_settings() -> Settings {
    match load_settings() {
        Ok(settings) => settings,
        Err(err) => {
            eprintln!("Failed to load settings: {err}");
            Settings::default()
        }
    }
}
