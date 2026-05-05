use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::course::CourseListResource;
use crate::state::GameState;
use super::resources::SelectedCourseID;
use super::selection_ui::{ConfirmButton, ConfirmButtonText, CourseListButton, ScrollContent};

pub fn scroll_system(
    mut wheel: MessageReader<MouseWheel>,
    mut query: Query<&mut Node, With<ScrollContent>>,
    mut offset: Local<f32>,
) {
    for ev in wheel.read() {
        *offset += ev.y * 20.0;
        *offset = offset.clamp(-1000.0, 1000.0);
        for mut node in &mut query {
            node.top = Val::Px(*offset);
        }
    }
}

pub fn update_course_list_buttons(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &CourseListButton),
        Changed<Interaction>,
    >,
    mut selected_id: ResMut<SelectedCourseID>,
) {
    for (interaction, mut background_color, button) in &mut button_query {
        match interaction {
            Interaction::Pressed => {
                selected_id.0 = Some(button.0);
                background_color.0 = Color::srgb(0.2, 0.2, 0.2);
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.0, 0.9, 0.0);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.1, 0.9, 0.2);
            }
        }
    }
}

pub fn update_confirm_button_text(
    mut text_query: Query<&mut Text, With<ConfirmButtonText>>,
    selected_id: Res<SelectedCourseID>,
    course_list_res: Res<CourseListResource>,
) {
    let name = if let Some(id) = selected_id.0 {
        course_list_res
            .0
            .iter()
            .find(|x| x.0.id == id)
            .map(|x| x.0.name.clone())
            .unwrap_or_else(|| "None".to_string())
    } else {
        "None".to_string()
    };
    for mut text in &mut text_query {
        **text = format!("決定: {}", name);
    }
}

pub fn update_confirm_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ConfirmButton>)>,
    selected_id: Res<SelectedCourseID>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if selected_id.0.is_none() {
        return;
    }
    for (interaction, mut background_color) in &mut button_query {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.2, 0.2, 0.5);
                game_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.0, 0.9, 0.9);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.1, 0.9, 0.9);
            }
        }
    }
}
