use crate::{config::GameConfig, state::GameState};
use bevy::prelude::*;

pub fn exit(
    mut game_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    config: Res<GameConfig>,
) {
    if keys.just_pressed(config.input.exit) {
        game_state.set(GameState::Loading);
    }
}
