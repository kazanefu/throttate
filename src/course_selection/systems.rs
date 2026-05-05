use super::resources::SelectedCourseID;
use super::selection_ui::{ConfirmButton, ConfirmButtonText, CourseListButton};
use crate::course::CourseListResource;
use crate::state::GameState;
use bevy::prelude::*;


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
    mut button_query: Query<(&Interaction, &mut BackgroundColor), With<ConfirmButton>>,
    selected_id: Res<SelectedCourseID>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let is_selected = selected_id.0.is_some();

    for (interaction, mut background_color) in &mut button_query {
        if !is_selected {
            background_color.0 = Color::srgb(0.5, 0.5, 0.5); // Gray
            continue;
        }

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
