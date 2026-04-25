use crate::FONT_PATH;

use super::*;
#[derive(Component)]
pub struct TextBox;

pub fn text_box_bundle(x: f32, y: f32, sentence: &str, asset_server: &AssetServer) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 5.0),
        TextBox,
        Text2d::new(sentence),
        TextFont {
            font: asset_server.load(FONT_PATH),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
    )
}