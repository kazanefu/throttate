use crate::{
    action_effect::FireDeathEffect,
    course::course_items::death_box::Death,
    hammer::definition::{Hammer, HammerFreeMessage, HammerState},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::get_player_collision;
use crate::playing::player::{DeathCount, Player, TargetCheckPoint};

pub fn handle_death(
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
        if let CollisionEvent::Started(e1, e2, _) = *event
            && let Some(other) = get_player_collision(e1, e2, player_entity)
                && death_query.get(other).is_ok() {
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
