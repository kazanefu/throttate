use bevy::prelude::*;
use crate::state::GameState;

pub fn exit(mut game_state: ResMut<NextState<GameState>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Loading);
    }
}