use bevy::prelude::*;

#[derive(Default, Clone, Copy, PartialEq,Eq,Hash,Debug,States)]
pub enum GameState {
    #[default]
    Start,
    CourseSelection,
    Playing,
    Result,
}