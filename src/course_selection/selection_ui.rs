use crate::course::CourseEntry;
use crate::{course::CourseListResource, state::GameState, *};
use super::systems::*;

pub struct SelectionUiPlugin;

impl Plugin for SelectionUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CourseSelection), spawn_selection_ui)
            .add_systems(
                Update,
                (
                    scroll_system,
                    update_course_list_buttons,
                    update_confirm_button_text,
                    update_confirm_button,
                )
                    .run_if(in_state(GameState::CourseSelection)),
            );
    }
}

const SELECTION_EXPLANATION: &str = r#"
プレイするコースを選択してください
"#;

#[derive(Component)]
pub struct ConfirmButton;

#[derive(Component)]
pub struct ConfirmButtonText;

#[derive(Component)]
pub struct ScrollContent;

#[derive(Component)]
pub struct CourseListButton(pub usize);

fn confirm_ui_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
        Button,
        ConfirmButton,
        Node {
            width: percent(50),
            height: percent(10),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        children![(
            ConfirmButtonText,
            Text::new("Confirm"),
            TextFont {
                font: asset_server.load(FONT_PATH),
                font_size: 40.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Center),
            TextColor(Color::srgb(0.0, 0.0, 0.0))
        )],
    )
}

fn selection_explanation_text_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
        Text::new(SELECTION_EXPLANATION),
        TextFont {
            font: asset_server.load(FONT_PATH),
            font_size: 40.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Center),
        TextColor::WHITE,
    )
}

fn selection_canvas_bundle() -> impl Bundle {
    (
        DespawnOnExit(GameState::CourseSelection),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            overflow: Overflow::clip(),
            ..default()
        },
    )
}

fn selection_sub_canvas_bundle() -> impl Bundle {
    (
        ScrollContent,
        Node {
            width: percent(100),
            height: percent(100),
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            top: Val::Px(0.0),
            row_gap: Val::Px(10.0),
            ..default()
        },
    )
}

fn course_list_button_node_bundle(len: usize) -> impl Bundle {
    (Node {
        width: percent(30),
        height: percent(12 * len),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(10.0),
        ..default()
    },)
}

fn course_list_button_bundle(
    asset_server: &AssetServer,
    course_entry: &CourseEntry,
    len: usize,
) -> impl Bundle {
    (
        Button,
        CourseListButton(course_entry.id),
        Node {
            width: percent(100),
            height: percent((100.0 - len as f32 * 2.0) / len as f32),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.9, 0.2)),
        children![(
            Text::new(&course_entry.name),
            TextFont {
                font: asset_server.load(FONT_PATH),
                font_size: 40.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Center),
            TextColor(Color::srgb(0.0, 0.0, 0.0))
        )],
    )
}

fn spawn_selection_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    course_list_res: Res<CourseListResource>,
) {
    let canvas = commands.spawn(selection_canvas_bundle()).id();
    let sub_canvas = commands.spawn(selection_sub_canvas_bundle()).id();
    let explanation_text = commands
        .spawn(selection_explanation_text_bundle(&asset_server))
        .id();
    let confirm_button = commands.spawn(confirm_ui_bundle(&asset_server)).id();
    let course_list_button = commands
        .spawn(course_list_button_node_bundle(course_list_res.0.len()))
        .id();
    let mut course_buttons = Vec::new();
    for (course_entry, _) in &course_list_res.0 {
        course_buttons.push(
            commands
                .spawn(course_list_button_bundle(
                    &asset_server,
                    course_entry,
                    course_list_res.0.len(),
                ))
                .id(),
        );
    }
    commands
        .entity(course_list_button)
        .add_children(&course_buttons);
    commands.entity(sub_canvas).add_children(&[
        explanation_text,
        confirm_button,
        course_list_button,
    ]);
    commands.entity(canvas).add_child(sub_canvas);
}
