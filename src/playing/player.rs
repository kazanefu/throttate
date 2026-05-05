use crate::{
    action_effect::{FireCheckPointEffect, FireDeathEffect},
    course::course_items::{checkpoint::CheckPoint, death_box::Death, goal::Goal},
    hammer::definition::{Hammer, HammerFreeMessage, HammerState},
    state::{GameState, RunningState},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ReachedGoalMessage>().add_systems(
            Update,
            (handle_death, reach_checkpoint, reach_goal, respawn)
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

/// Returns the other entity if one of them is the player.
fn get_player_collision(e1: Entity, e2: Entity, player: Entity) -> Option<Entity> {
    if e1 == player {
        Some(e2)
    } else if e2 == player {
        Some(e1)
    } else {
        None
    }
}

fn handle_death(
    mut commands: Commands,
    mut hammer_query: Query<&mut Hammer>,
    mut player_query: Query<(&mut DeathCount, &mut Transform, &TargetCheckPoint, Entity), With<Player>>,
    death_query: Query<&Death>,
    mut collision_event: MessageReader<CollisionEvent>,
    mut hammer_action_writer: MessageWriter<HammerFreeMessage>,
    mut death_writer: MessageWriter<FireDeathEffect>,
) {
    let Ok((mut death_count, mut transform, checkpoint, player_entity)) = player_query.single_mut() else {
        return;
    };

    for event in collision_event.read() {
        if let CollisionEvent::Started(e1, e2, _) = *event {
            if let Some(other) = get_player_collision(e1, e2, player_entity) {
                if death_query.get(other).is_ok() {
                    // Reset hammer
                    hammer_action_writer.write(HammerFreeMessage);
                    for mut hammer in &mut hammer_query {
                        if matches!(hammer.state, HammerState::Spinning) {
                            commands.entity(player_entity).remove::<ImpulseJoint>();
                            hammer.state = HammerState::Flying;
                        }
                    }
                    // Fire effect and respawn
                    death_writer.write(FireDeathEffect(transform.translation));
                    death_count.0 += 1;
                    transform.translation = checkpoint.position;
                }
            }
        }
    }
}

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

fn reach_checkpoint(
    mut player_query: Query<(Entity, &mut TargetCheckPoint), With<Player>>,
    mut collision_event: MessageReader<CollisionEvent>,
    checkpoint_query: Query<(&CheckPoint, &Transform)>,
    mut checkpoint_effect_writer: MessageWriter<FireCheckPointEffect>,
) {
    let Ok((player_entity, mut target_checkpoint)) = player_query.single_mut() else {
        return;
    };

    for event in collision_event.read() {
        if let CollisionEvent::Started(e1, e2, _) = *event {
            if let Some(other) = get_player_collision(e1, e2, player_entity) {
                if let Ok((checkpoint, transform)) = checkpoint_query.get(other) {
                    if checkpoint.priority() >= target_checkpoint.priority {
                        let prev_position = target_checkpoint.position;
                        target_checkpoint.priority = checkpoint.priority();
                        target_checkpoint.position = transform.translation;
                        
                        if prev_position != target_checkpoint.position {
                            checkpoint_effect_writer.write(FireCheckPointEffect(target_checkpoint.position));
                        }
                    }
                }
            }
        }
    }
}

fn reach_goal(
    mut reach_message: MessageWriter<ReachedGoalMessage>,
    mut collision_event: MessageReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    goal_query: Query<(), With<Goal>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    for event in collision_event.read() {
        if let CollisionEvent::Started(e1, e2, _) = *event {
            if let Some(other) = get_player_collision(e1, e2, player_entity) {
                if goal_query.get(other).is_ok() {
                    reach_message.write(ReachedGoalMessage);
                    println!("goal reached!");
                }
            }
        }
    }
}
