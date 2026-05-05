#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_hanabi::prelude::*;

mod config;
mod course;
mod course_selection;
mod hammer;
mod start;
mod state;
mod utils;
mod playing;
mod result;
mod action_effect;
mod ui_utils;

pub use config::*;
pub use utils::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        watch_for_changes_override: Some(true),
        ..default()
    }));
    bevy::asset::embedded_asset!(app, "fonts/NotoSansJP-Bold.ttf");
    bevy::asset::embedded_asset!(app, "images/bluepivot.png");
    bevy::asset::embedded_asset!(app, "images/magentapivot.png");
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(HanabiPlugin)
        .add_plugins(utils::UtilityPlugin)
        .add_plugins(ui_utils::UiUtilsPlugin)
        .init_state::<state::GameState>()
        .init_state::<state::RunningState>()
        .init_resource::<GameplayConfig>()
        .add_plugins(course::CoursePlugin)
        .add_plugins(hammer::HammerPlugin)
        .add_plugins(start::StartPlugin)
        .add_plugins(playing::PlayingPlugin)
        .add_plugins(course_selection::CourseSelectionPlugin)
        .add_plugins(result::ResultPlugin)
        .add_plugins(action_effect::ActionEffectPlugin)
        .run();
}

#[allow(unused)]
fn setup(mut commands: Commands, config: Res<GameplayConfig>) {
    // カメラ
    commands.spawn(Camera2d);

    // 床
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(config.floor_size.x / 2.0, config.floor_size.y / 2.0),
        Transform::from_translation(config.floor_position),
        Sprite {
            color: config.floor_color,
            custom_size: Some(config.floor_size),
            ..default()
        },
    ));

    let _hammer = hammer::spawn_hammer(&mut commands, config.initial_hammer_position);
}
