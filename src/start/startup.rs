use crate::state::GameState;
use crate::utils::state_camera_bundle;
use bevy::prelude::Commands;

pub fn spawn_start_camera(mut commands: Commands) {
    commands.spawn(state_camera_bundle(GameState::Start));
}
