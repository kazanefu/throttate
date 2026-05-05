use bevy::prelude::*;

#[derive(Resource)]
pub struct GameplayConfig {
    pub initial_hammer_position: Vec2,
    pub floor_size: Vec2,
    pub floor_position: Vec3,
    pub floor_color: Color,
}

impl Default for GameplayConfig {
    fn default() -> Self {
        Self {
            initial_hammer_position: Vec2::new(0.0, 0.0),
            floor_size: Vec2::new(400.0, 20.0),
            floor_position: Vec3::new(0.0, -100.0, 0.0),
            floor_color: Color::srgb(0.5, 0.5, 0.2),
        }
    }
}
