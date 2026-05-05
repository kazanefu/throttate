use crate::playing::DeathCount;
use crate::playing::Player;
use crate::playing::score::Score;
use crate::state::*;
use crate::utils::*;
use bevy::prelude::*;

pub struct PlayingUiPlugin;

impl Plugin for PlayingUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_playing_ui)
            .add_systems(
                Update,
                (update_time_ui, update_death_count_ui)
                    .run_if(in_state(GameState::Playing).and(in_state(RunningState::Running))),
            );
    }
}

#[derive(Component)]
struct TimeUi;

#[derive(Component)]
struct DeathCountUi;

fn time_ui_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
        Text::new(""),
        TimeUi,
        TextFont {
            font: asset_server.load(FONT_PATH),
            font_size: 40.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Center),
        TextColor::WHITE,
    )
}

fn death_count_ui_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
        Text::new(""),
        DeathCountUi,
        TextFont {
            font: asset_server.load(FONT_PATH),
            font_size: 40.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Center),
        TextColor::WHITE,
    )
}

use crate::ui_utils::*;

fn spawn_playing_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let canvas = commands.spawn(root_canvas_bundle(GameState::Playing)).id();
    commands.entity(canvas).insert(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::FlexEnd,
        justify_content: JustifyContent::FlexStart,
        flex_direction: FlexDirection::Column,
        ..default()
    });
    let time_ui = commands.spawn(time_ui_bundle(&asset_server)).id();
    let stopwatch = commands.spawn(StopWatch::new(true)).id();
    commands.entity(time_ui).add_child(stopwatch);
    let death_count_ui = commands.spawn(death_count_ui_bundle(&asset_server)).id();
    commands
        .entity(canvas)
        .add_children(&[time_ui, death_count_ui]);
}

fn update_time_ui(
    mut time_ui_query: Query<&mut Text, With<TimeUi>>,
    stopwatch_query: Query<&StopWatch>,
    mut time_score: ResMut<Score>,
) {
    let Ok(stopwatch) = stopwatch_query.single() else {
        warn!("Found none or multiple stopwatch in world");
        return;
    };
    let time = stopwatch.now();
    for mut time_ui_text in &mut time_ui_query {
        **time_ui_text = format!("タイム: {:.2}", time);
        time_score.time = time;
    }
}

fn update_death_count_ui(
    mut death_count_ui_query: Query<&mut Text, With<DeathCountUi>>,
    death_count_query: Query<&DeathCount, (With<Player>, Changed<DeathCount>)>,
    mut death_score: ResMut<Score>,
) {
    let Ok(death_count) = death_count_query.single() else {
        return;
    };
    let count = death_count.0;
    for mut text in &mut death_count_ui_query {
        **text = format!("死亡数 {}", count);
        death_score.death = count;
    }
}
