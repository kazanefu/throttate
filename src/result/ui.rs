use crate::FONT_PATH;
use crate::playing::score::Score;
use crate::state::GameState;
use crate::ui_utils::*;
use bevy::prelude::*;

pub struct ResultUiPlugin;

impl Plugin for ResultUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Result), spawn_result_ui)
            .add_systems(
                Update,
                (
                    generic_button_system::<ContinueButton>(
                        Color::srgb(0.5, 0.5, 0.2),
                        Color::srgb(0.8, 0.8, 0.2),
                        Color::srgb(1.0, 1.0, 0.2),
                    ),
                    update_continue_button_logic,
                )
                    .run_if(in_state(GameState::Result)),
            );
    }
}

#[derive(Component)]
struct ContinueButton;

fn result_text_bundle(asset_server: &AssetServer, score: &Score) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Node::default(),
                children![
                    (
                        Text::new("タイム: "),
                        TextFont {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ),
                    (
                        Text::new(format!("{:.2}", score.time)),
                        TextFont {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.0, 0.9, 1.0)),
                    ),
                ]
            ),
            (
                Node::default(),
                children![
                    (
                        Text::new("死亡数: "),
                        TextFont {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ),
                    (
                        Text::new(format!("{}", score.death)),
                        TextFont {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.3, 0.0)),
                    ),
                ]
            ),
        ],
    )
}

fn spawn_result_ui(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    let canvas = commands.spawn(root_canvas_bundle(GameState::Result)).id();
    let text = commands
        .spawn(result_text_bundle(&asset_server, &score))
        .id();
    let continue_button = commands
        .spawn((
            button_bundle(
                &asset_server,
                "コンティニュー",
                Val::Percent(40.0),
                Val::Percent(10.0),
                40.0,
                Color::srgb(0.5, 0.5, 0.2),
                Color::srgb(0.0, 0.0, 0.0),
            ),
            ContinueButton,
        ))
        .id();
    commands
        .entity(canvas)
        .add_children(&[text, continue_button]);
}

fn update_continue_button_logic(
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>,
) {
    for interaction in &query {
        if matches!(interaction, Interaction::Pressed) {
            game_state.set(GameState::CourseSelection);
        }
    }
}
