use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Copy)]
pub enum HammerState {
    Spinning,
    Flying,
}

#[derive(Message)]
pub struct ChangeHandleDirection(pub HandleDirection);

#[derive(Clone, Copy)]
pub enum HandleDirection {
    LeftLeft,
    RightRight,
    LeftRight,
    RightLeft,
}

impl HandleDirection {
    pub fn offset(&self, config: &crate::config::HammerConfig) -> Vec2 {
        match self {
            Self::LeftLeft => config.handle_offset,
            Self::RightRight => config.handle_offset * -1.0,
            Self::LeftRight => config.handle_offset,
            Self::RightLeft => config.handle_offset * -1.0,
        }
    }
    pub fn spin(&self, config: &crate::config::HammerConfig) -> (f32, f32) {
        match self {
            Self::LeftLeft => (config.spin_velocity, config.spin_stiffness),
            Self::RightRight => (-config.spin_velocity, config.spin_stiffness),
            Self::LeftRight => (-config.spin_velocity, config.spin_stiffness),
            Self::RightLeft => (config.spin_velocity, config.spin_stiffness),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Hammer {
    pub pivot_entity: Entity,
    pub state: HammerState,
    pub handle_direction: HandleDirection,
}

#[derive(Component)]
pub struct Pivot;

#[derive(Resource)]
pub struct PivotTextures {
    pub blue: Handle<Image>,
    pub magenta: Handle<Image>,
}

#[derive(Message)]
pub struct HammerActionMessage;

#[derive(Message)]
pub struct HammerFreeMessage;

pub fn hammer_bundle(
    pivot_entity: Entity,
    translate: Vec2,
    config: &crate::config::HammerConfig,
) -> impl Bundle {
    (
        Hammer {
            pivot_entity,
            state: HammerState::Spinning,
            handle_direction: HandleDirection::LeftLeft,
        },
        RigidBody::Dynamic,
        Transform::from_xyz(translate.x, translate.y, 10.0),
        Collider::ball(config.size),
        Restitution::coefficient(0.8),
        Velocity::default(),
        Ccd::enabled(),
        ImpulseJoint::new(
            pivot_entity,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::ZERO)
                .local_anchor2(config.handle_offset)
                .motor_velocity(config.spin_velocity, config.spin_stiffness),
        ),
        Sprite {
            color: Color::srgb(0.0, 0.4, 0.9),
            custom_size: Some(Vec2::new(config.size * 2.0, config.size * 2.0)),
            ..default()
        },
        children![
            (
                Transform::from_xyz(config.handle_offset.x, config.handle_offset.y, 10.0),
                Sprite {
                    color: Color::srgb(0.0, 0.9, 0.9),
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
            ),
            (
                Transform::from_xyz(-config.handle_offset.x, config.handle_offset.y, 10.0),
                Sprite {
                    color: Color::srgb(0.9, 0.0, 0.9),
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
            ),
        ],
    )
}
