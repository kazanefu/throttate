use bevy::prelude::*;

pub mod checkpoint;
pub mod death;
pub mod goal;

/// Returns the other entity if one of them is the player.
pub fn get_player_collision(e1: Entity, e2: Entity, player: Entity) -> Option<Entity> {
    if e1 == player {
        Some(e2)
    } else if e2 == player {
        Some(e1)
    } else {
        None
    }
}
