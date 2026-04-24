use crate::{course::course_items::{checkpoint::CheckPoint, death_box::Death, goal::Goal}, state::{GameState, RunningState}};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_death.run_if(in_state(GameState::Playing).and(in_state(RunningState::Running))));
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

fn handle_death(
    mut player_query: Query<
        (&mut DeathCount, &mut Transform, &TargetCheckPoint, Entity),
        With<Player>,
    >,
    death_query: Query<&Death>,
    mut collision_event: MessageReader<CollisionEvent>,
) {
    let mut player = player_query
        .single_mut()
        .expect("found none or multiple player in the world");
    for event in collision_event.read() {
        match *event {
            CollisionEvent::Started(e1, e2, _) => {
                if (player.3 == e1 && death_query.get(e2).is_ok())
                    || (player.3 == e2 && death_query.get(e1).is_ok())
                {
                    player.0.0 += 1;
                    player.1.translation = player.2.position;
                }
            }
            CollisionEvent::Stopped(_e1, _e2, _) => {}
        }
    }
}

