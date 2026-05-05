use crate::{
    hammer::definition::{Hammer, HammerFreeMessage, HammerState},
    state::{GameState, RunningState},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod collision;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ReachedGoalMessage>().add_systems(
            Update,
            (
                collision::death::handle_death,
                collision::checkpoint::reach_checkpoint,
                collision::goal::reach_goal,
                respawn,
            )
                .run_if(in_state(GameState::Playing).and(in_state(RunningState::Running))),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct DeathCount(pub u32);

#[derive(Component, Default)]
pub struct TargetCheckPoint {
    pub position: Vec3,
    pub priority: u32,
}

#[derive(Message)]
pub struct ReachedGoalMessage;

fn respawn(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &TargetCheckPoint, Entity, &mut Hammer)>,
    mut hammer_action_writer: MessageWriter<HammerFreeMessage>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }
    for (mut transform, checkpoint, entity, mut hammer) in &mut player_query {
        hammer_action_writer.write(HammerFreeMessage);
        if matches!(hammer.state, HammerState::Spinning) {
            commands.entity(entity).remove::<ImpulseJoint>();
            hammer.state = HammerState::Flying;
        }
        transform.translation = checkpoint.position;
    }
}
