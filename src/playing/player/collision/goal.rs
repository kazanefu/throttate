use crate::course::course_items::goal::Goal;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::get_player_collision;
use crate::playing::player::{Player, ReachedGoalMessage};

pub fn reach_goal(
    mut reach_message: MessageWriter<ReachedGoalMessage>,
    mut collision_event: MessageReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    goal_query: Query<(), With<Goal>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    for event in collision_event.read() {
        if let CollisionEvent::Started(e1, e2, _) = *event
            && let Some(other) = get_player_collision(e1, e2, player_entity)
                && goal_query.get(other).is_ok() {
                    reach_message.write(ReachedGoalMessage);
                    println!("goal reached!");
                }
    }
}
