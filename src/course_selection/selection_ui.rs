use crate::course::CourseEntry;
use crate::{course::CourseListResource, state::GameState, *};
use crate::ui_utils::*;
use super::systems::*;

pub struct SelectionUiPlugin;

impl Plugin for SelectionUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CourseSelection), spawn_selection_ui)
            .add_systems(
                Update,
                (
                    update_course_list_buttons,
                    update_confirm_button_text,
                    update_confirm_button,
                    generic_button_system::<ConfirmButton>(
                        Color::srgb(0.1, 0.9, 0.9),
                        Color::srgb(0.0, 0.9, 0.9),
                        Color::srgb(0.2, 0.2, 0.5),
                    ),
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
pub struct CourseListButton(pub usize);

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

fn course_list_button_node_bundle(len: usize) -> impl Bundle {
    (Node {
        width: Val::Percent(30.0),
        height: Val::Percent(12.0 * len as f32),
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
            width: Val::Percent(100.0),
            height: Val::Percent((100.0 - len as f32 * 2.0) / len as f32),
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
    let canvas = commands.spawn(root_canvas_bundle(GameState::CourseSelection)).id();
    let sub_canvas = commands.spawn(scrolling_content_bundle()).id();
    let explanation_text = commands
        .spawn(selection_explanation_text_bundle(&asset_server))
        .id();
    
    let confirm_button = commands.spawn((
        Button,
        ConfirmButton,
        Node {
            width: Val::Percent(50.0),
            height: Val::Percent(10.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.9, 0.9)),
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
    )).id();
    
    let course_list_button_node = commands
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
        .entity(course_list_button_node)
        .add_children(&course_buttons);
    commands.entity(sub_canvas).add_children(&[
        explanation_text,
        confirm_button,
        course_list_button_node,
    ]);
    commands.entity(canvas).add_child(sub_canvas);

    // Tag the text child of confirm_button with ConfirmButtonText
    // We can't easily do it with the current button_bundle.
    // I'll just spawn the button manually here to keep it simple for now, or update ui_utils.
}
