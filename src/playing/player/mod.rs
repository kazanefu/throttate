use crate::{
    action_effect::{FireCheckPointEffect, FireDeathEffect},
    course::course_items::{checkpoint::CheckPoint, death_box::Death, goal::Goal},
    hammer::definition::{Hammer, HammerFreeMessage, HammerState},
    state::GameState,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
mod collision;
mod input;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ReachedGoalMessage>()
            .add_plugins(collision::PlayerCollisonPlugin)
            .add_plugins(input::PlayerInputPlugin)
            .add_systems(Update, respawn.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct DeathCount(pub u32);

impl DeathCount {
    pub fn count_up(&mut self) {
        self.0 += 1;
    }
}

#[derive(Component, Default)]
pub struct TargetCheckPoint {
    pub position: Vec3,
    pub priority: u32,
}

#[derive(Message)]
pub struct ReachedGoalMessage;

fn respawn(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &TargetCheckPoint, Entity, &mut Hammer), With<Player>>,
    mut hammer_action_writer: MessageWriter<HammerFreeMessage>,
    mut respawn: MessageReader<input::RespawnMessage>,
) {
    for _ in respawn.read() {
        for (mut transform, checkpoint, entity, mut hammer) in &mut player_query {
            hammer_action_writer.write(HammerFreeMessage);
            if matches!(hammer.state, HammerState::Spinning) {
                commands.entity(entity).remove::<ImpulseJoint>();
                hammer.state = HammerState::Flying;
            }
            transform.translation = checkpoint.position;
        }
    }
}
