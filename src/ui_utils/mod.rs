use bevy::prelude::*;
use crate::utils::FONT_PATH;
use bevy::input::mouse::MouseWheel;

pub struct UiUtilsPlugin;

impl Plugin for UiUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, scroll_system);
    }
}

pub type InteractionQuery<'w, 's, T> =
    Query<'w, 's, (&'static Interaction, &'static mut BackgroundColor), (Changed<Interaction>, With<T>)>;

/// A generic system for button interaction visuals.
pub fn generic_button_system<T: Component>(
    normal_color: Color,
    hover_color: Color,
    pressed_color: Color,
) -> impl FnMut(InteractionQuery<T>) {
    move |mut query| {
        for (interaction, mut background_color) in &mut query {
            match interaction {
                Interaction::Pressed => {
                    background_color.0 = pressed_color;
                }
                Interaction::Hovered => {
                    background_color.0 = hover_color;
                }
                Interaction::None => {
                    background_color.0 = normal_color;
                }
            }
        }
    }
}

#[derive(Component)]
pub struct ScrollContent;

pub fn scroll_system(
    mut wheel: MessageReader<MouseWheel>,
    mut query: Query<&mut Node, With<ScrollContent>>,
    mut offset: Local<f32>,
) {
    for ev in wheel.read() {
        *offset += ev.y * 20.0;
        // Clamp might need to be customizable, but 1000 is a safe default for now
        *offset = offset.clamp(-1000.0, 1000.0);
        for mut node in &mut query {
            node.top = Val::Px(*offset);
        }
    }
}

pub fn root_canvas_bundle(despawn_state: impl States) -> impl Bundle {
    (
        crate::DespawnOnExit(despawn_state),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            overflow: Overflow::clip(),
            ..default()
        },
    )
}

pub fn scrolling_content_bundle() -> impl Bundle {
    (
        ScrollContent,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
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

pub fn button_bundle(
    asset_server: &AssetServer,
    text: &str,
    width: Val,
    height: Val,
    font_size: f32,
    bg_color: Color,
    text_color: Color,
) -> impl Bundle {
    (
        Button,
        Node {
            width,
            height,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(bg_color),
        children![(
            Text::new(text),
            TextFont {
                font: asset_server.load(FONT_PATH),
                font_size,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Center),
            TextColor(text_color)
        )],
    )
}
