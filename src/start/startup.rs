use crate::state::GameState;
use bevy::prelude::*;

fn start_cammera_bundle() -> impl Bundle {
    (Camera2d, DespawnOnExit(GameState::Start))
}

pub fn spawn_start_cammera(mut commands: Commands) {
    commands.spawn(start_cammera_bundle());

}
