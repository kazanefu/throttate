use bevy::prelude::*;

use crate::{playing::ReachedGoalMessage, state::GameState};


pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::default()).add_systems(Update, handle_goal);
    }
}

#[derive(Resource,Default)]
pub struct Score {
    pub time: f32,
    pub death: u32,
}

fn handle_goal(mut goal_message: MessageReader<ReachedGoalMessage>, mut game_state: ResMut<NextState<GameState>>) {
    for _ in goal_message.read() {
        game_state.set(GameState::Result);
    }
}


