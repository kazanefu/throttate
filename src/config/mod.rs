use bevy::prelude::*;

mod input;

#[derive(Resource, Default, Clone)]
pub struct GameConfig {
    pub input: input::InputSetting,
}
