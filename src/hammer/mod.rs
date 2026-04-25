use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub mod definition;
mod systems;
use crate::state::RunningState;
use definition::*;
use systems::*;

pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<HammerActionMessage>()
            .add_message::<ChangeHandleDirection>()
            .add_message::<HammerFreeMessage>()
            .add_systems(
                Update,
                (
                    handle_hammer_input,
                    update_hammer,
                    free_hammer,
                    change_handle_direction,
                )
                    .run_if(in_state(RunningState::Running)),
            );
    }
}

#[allow(unused)]
pub fn spawn_hammer<'a>(commands: &'a mut Commands, translate: Vec2) -> EntityCommands<'a> {
    let pivot = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(translate.x - 1.0, translate.y - 1.0, 0.0),
            Pivot,
            Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
        ))
        .id();
    commands.spawn(hammer_bundle(pivot, translate))
}
