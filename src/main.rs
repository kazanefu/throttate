#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_rapier2d::prelude::*;

mod action_effect;
mod config;
mod course;
mod course_selection;
mod hammer;
mod playing;
mod result;
mod start;
mod state;
mod utils;

pub use utils::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    bevy::asset::embedded_asset!(app, "fonts/NotoSansJP-Bold.ttf");
    bevy::asset::embedded_asset!(app, "images/bluepivot.png");
    bevy::asset::embedded_asset!(app, "images/magentapivot.png");
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(HanabiPlugin)
        .init_resource::<config::GameConfig>()
        .add_plugins(utils::UtilityPlugin)
        .init_state::<state::GameState>()
        .init_state::<state::RunningState>()
        .add_plugins(course::CoursePlugin)
        .add_plugins(hammer::HammerPlugin)
        .add_plugins(start::StartPlugin)
        .add_plugins(playing::PlayingPlugin)
        .add_plugins(course_selection::CourseSelectionPlugin)
        .add_plugins(result::ResultPlugin)
        .add_plugins(action_effect::ActionEffectPlugin)
        .run();
}
