use crate::{LifeTime, course::course_items::death_box::Death};

use super::*;

pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, turret_shot);
    }
}

#[derive(Component)]
pub struct Turret;

pub fn turret_bundle(
    x: f32,
    y: f32,
    interval: f32,
    rotation: f32,
    box_size: f32,
) -> impl Bundle {
    (
        Transform {
            translation: Vec3::new(x, y, 0.0),
            rotation: Quat::from_rotation_z(rotation),
            scale: Vec3::ONE,
        },
        crate::utils::Interval {
            time: 0.0,
            interval,
        },
        Turret,
        RigidBody::Fixed,
        Collider::cuboid(box_size / 2.0, box_size / 2.0),
        Sprite {
            color: Color::srgb(0.8, 0.4, 0.2),
            custom_size: Some(Vec2::new(box_size, box_size)),
            ..default()
        },
    )
}

const BULLET_LIFE_TIME: f32 = 3.0;
const BULLET_SPEED: f32 = 500.0;
#[derive(Component)]
struct TurretBullet;

fn bullet_bundle(translation: Vec3, rotation: Quat, box_size: f32) -> impl Bundle {
    let dir = (rotation * Vec3::X).truncate();
    (
        TurretBullet,
        Death,
        Transform {
            translation,
            rotation,
            scale: Vec3::ONE,
        },
        GlobalTransform::default(),
        LifeTime::new(BULLET_LIFE_TIME),
        RigidBody::Dynamic,
        Collider::cuboid(box_size / 4.0, box_size / 4.0),
        Sprite {
            color: Color::srgb(0.9, 0.2, 0.2),
            custom_size: Some(Vec2::new(box_size / 2.0, box_size / 2.0)),
            ..default()
        },
        Velocity {
            linvel: dir * BULLET_SPEED,
            angvel: 0.0,
        },
    )
}

fn turret_shot(
    mut commands: Commands,
    mut turret_query: Query<(&Transform, &mut crate::utils::Interval), With<Turret>>,
    config: Res<crate::config::GameConfig>,
) {
    let box_size = config.course.one_box_size;
    for (turret_transform, mut turret_interval) in &mut turret_query {
        if turret_interval.is_ready() {
            turret_interval.reset();
            commands.spawn(bullet_bundle(
                turret_transform.translation + turret_transform.rotation * Vec3::X * box_size,
                turret_transform.rotation,
                box_size,
            ));
        }
    }
}
