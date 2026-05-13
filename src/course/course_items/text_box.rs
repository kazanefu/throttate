use super::*;
#[derive(Component)]
pub struct TextBox;

pub fn text_box_bundle(x: f32, y: f32, sentence: &str, font: &Handle<Font>) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 5.0),
        TextBox,
        Text2d::new(sentence),
        TextFont {
            font: font.clone(),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
    )
}
