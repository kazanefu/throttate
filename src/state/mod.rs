use bevy::prelude::*;

#[derive(Default, Clone, Copy, PartialEq,Eq,Hash,Debug,States)]
pub enum GameState {
    #[default]
    Loading,
    Start,
    CourseSelection,
    Playing,
    Result,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum RunningState {
    #[default]
    Running,
    #[allow(unused)]
    Paused,
}