use super::*;
use crate::course::SpawnCourseMessage;
use crate::course_selection::resources::SelectedCourseID;
use crate::hammer::definition::{ChangeHandleDirection, HandleDirection};
use crate::hammer::spawn_hammer;
use crate::state::GameState;
use bevy_rapier2d::prelude::*;
pub struct PlayingStartupPlugin;

impl Plugin for PlayingStartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            ((spawn_selected_course,).chain(), spawn_player),
        );
    }
}

fn spawn_selected_course(
    mut spawn_message: MessageWriter<SpawnCourseMessage>,
    id: Res<SelectedCourseID>,
) {
    spawn_message.write(SpawnCourseMessage(
        id.0.expect("you must have selected course"),
    ));
}

fn spawn_player(
    mut commands: Commands,
    mut handle_direction_message: MessageWriter<ChangeHandleDirection>,
    config: Res<crate::config::GameplayConfig>,
) {
    let entities = spawn_hammer(&mut commands, config.initial_hammer_position);
    commands.entity(entities.hammer).insert((
        Player,
        DeathCount(0),
        TargetCheckPoint::default(),
        ActiveEvents::COLLISION_EVENTS,
        DespawnOnExit(GameState::Playing),
    ));
    commands.entity(entities.pivot).insert(DespawnOnExit(GameState::Playing));
    
    handle_direction_message.write(ChangeHandleDirection(HandleDirection::LeftLeft));
    commands.spawn(main_camera_bundle(entities.hammer));
}
