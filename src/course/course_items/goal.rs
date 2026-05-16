use super::*;
#[derive(Component)]
pub struct Goal;

pub fn goal_bundle(x: f32, y: f32, box_size: f32) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        Goal,
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(box_size / 2.0, box_size / 2.0),
        Sprite {
            color: Color::srgb(0.9, 0.2, 0.9),
            custom_size: Some(Vec2::new(box_size, box_size)),
            ..default()
        },
    )
}
