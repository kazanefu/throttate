use crate::{
    action_effect::FireCheckPointEffect,
    course::course_items::checkpoint::CheckPoint,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::get_player_collision;
use crate::playing::player::{Player, TargetCheckPoint};

pub fn reach_checkpoint(
    mut player_query: Query<(Entity, &mut TargetCheckPoint), With<Player>>,
    mut collision_event: MessageReader<CollisionEvent>,
    checkpoint_query: Query<(&CheckPoint, &Transform)>,
    mut checkpoint_effect_writer: MessageWriter<FireCheckPointEffect>,
) {
    let Ok((player_entity, mut target_checkpoint)) = player_query.single_mut() else {
        return;
    };

    for event in collision_event.read() {
        if let CollisionEvent::Started(e1, e2, _) = *event
            && let Some(other) = get_player_collision(e1, e2, player_entity)
                && let Ok((checkpoint, transform)) = checkpoint_query.get(other)
                    && checkpoint.priority() >= target_checkpoint.priority {
                        let prev_position = target_checkpoint.position;
                        target_checkpoint.priority = checkpoint.priority();
                        target_checkpoint.position = transform.translation;
                        
                        if prev_position != target_checkpoint.position {
                            checkpoint_effect_writer.write(FireCheckPointEffect(target_checkpoint.position));
                        }
                    }
    }
}
