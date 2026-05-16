use crate::collision::get_contained_entity;

use super::*;

pub struct SpeedUpPlugin;

impl Plugin for SpeedUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, speedup_system);
    }
}

#[derive(Component)]
pub struct SpeedUp {
    rate: f32,
}

impl SpeedUp {
    pub fn new(rate: f32) -> Self {
        Self { rate }
    }
    pub fn rate(&self) -> f32 {
        self.rate
    }
}

pub fn speedup_bundle(x: f32, y: f32, rate: f32) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        SpeedUp::new(rate),
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(ONE_BOX_SIZE / 2.0, ONE_BOX_SIZE / 2.0),
        Sensor,
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.3),
            custom_size: Some(Vec2::new(ONE_BOX_SIZE, ONE_BOX_SIZE)),
            ..default()
        },
    )
}

fn speedup_system(
    speedup_que: Query<&SpeedUp>,
    mut velocity_que: Query<&mut Velocity>,
    mut collision_events: MessageReader<CollisionEvent>,
) {
    for &event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let Some(speedup_entity) = get_contained_entity(e1, e2, &speedup_que) else {
                continue;
            };
            let Some(other) = get_contained_entity(e1, e2, &velocity_que) else {
                continue;
            };
            // unwrap: because it has been confirmed that querys contains entitys
            let mut velocity = velocity_que.get_mut(other).unwrap();
            let speedup = speedup_que.get(speedup_entity).unwrap();
            let next_velocity = velocity.linvel * speedup.rate();
            if next_velocity.length_squared() <= 10000.0 * 10000.0 {
                velocity.linvel = next_velocity
            };
        }
    }
}
