use bevy::ecs::query::{QueryData, QueryFilter};
use bevy::prelude::*;
pub fn get_contained_entity<Q: QueryData, F: QueryFilter>(
    e1: Entity,
    e2: Entity,
    query: &Query<Q, F>,
) -> Option<Entity> {
    if query.contains(e1) {
        Some(e1)
    } else if query.contains(e2) {
        Some(e2)
    } else {
        None
    }
}
