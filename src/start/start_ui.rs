use crate::state::GameState;
use crate::utils::FONT_PATH;
use crate::ui_utils::*;
use bevy::prelude::*;

const EXPLANATION_TEXT: &str = r#"
概要: 
    ハンマー投のようにぐるぐる回してから離すことで移動してゴールを目指すゲームです。
登場するもの:
    プレイヤー:
        青い立方体の見た目で操作可能
    赤色の立方体:
        当たると死ぬ
    水色の立方体:
        チェックポイント
    オレンジ色の立方体:
        小さな赤色の立方体を発射する
    黄色の立方体:
        一定速度以上の物体が当たると壊れる
    マゼンタの立方体:
        ゴール
    黄土色:
        地形
操作方法:
    Spaceキーで拘束して回転と拘束を解くのを切り替える
    矢印キーで回転軸の相対座標と回転方向を切り替える
    Rキーでチェックポイントに戻る
"#;

pub struct StartUiPlugin;

impl Plugin for StartUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), spawn_start_ui)
            .add_systems(
                Update,
                (
                    generic_button_system::<StartButton>(
                        Color::srgb(0.0, 0.5, 0.5),
                        Color::srgb(0.1, 0.8, 0.4),
                        Color::srgb(0.2, 1.0, 0.3),
                    ),
                    update_start_button_logic,
                )
                    .run_if(in_state(GameState::Start)),
            );
    }
}

#[derive(Component)]
struct StartButton;

fn explanation_text_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
        Text::new(EXPLANATION_TEXT),
        TextFont {
            font: asset_server.load(FONT_PATH),
            font_size: 40.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Left),
        TextColor::WHITE,
    )
}

fn spawn_start_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let canvas = commands.spawn(root_canvas_bundle(GameState::Start)).id();
    let sub_canvas = commands.spawn(scrolling_content_bundle()).id();
    let explanation_text = commands.spawn(explanation_text_bundle(&asset_server)).id();
    let start_button = commands
        .spawn((
            button_bundle(
                &asset_server,
                "スタート",
                Val::Percent(20.0),
                Val::Percent(10.0),
                40.0,
                Color::srgb(0.0, 0.5, 0.5),
                Color::srgb(0.2, 0.2, 0.2),
            ),
            StartButton,
        ))
        .id();
    commands
        .entity(sub_canvas)
        .add_children(&[explanation_text, start_button]);
    commands.entity(canvas).add_child(sub_canvas);
}

fn update_start_button_logic(
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    key: Res<ButtonInput<KeyCode>>,
) {
    for interaction in &query {
        if matches!(interaction, Interaction::Pressed) {
            game_state.set(GameState::CourseSelection);
        }
    }
    if key.just_pressed(KeyCode::Enter) {
        game_state.set(GameState::CourseSelection);
    }
}
