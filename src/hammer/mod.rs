use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub mod definition;
mod systems;
mod trail_effect;
use crate::state::RunningState;
use definition::*;
use systems::*;

pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<HammerActionMessage>()
            .add_message::<ChangeHandleDirection>()
            .add_message::<HammerFreeMessage>()
            .add_systems(Startup, load_pivot_texture)
            .add_systems(
                Update,
                (
                    handle_hammer_input,
                    update_hammer,
                    free_hammer,
                    change_handle_direction,
                    pivot_texture,
                    trail_effect::attach_trail_effect,
                    fix_hammer_z,
                )
                    .run_if(in_state(RunningState::Running)),
            );
    }
}

pub struct HammerEntities {
    pub hammer: Entity,
    pub pivot: Entity,
}

#[allow(unused)]
pub fn spawn_hammer(commands: &mut Commands, translate: Vec2) -> HammerEntities {
    let pivot = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(translate.x - 1.0, translate.y - 1.0, 10.0),
            Pivot,
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
        ))
        .id();
    let hammer = commands.spawn(hammer_bundle(pivot, translate)).id();
    HammerEntities { hammer, pivot }
}

fn load_pivot_texture(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PivotTextures {
        blue: asset_server.load("embedded://throtate/images/bluepivot.png"),
        magenta: asset_server.load("embedded://throtate/images/magentapivot.png"),
    });
}
