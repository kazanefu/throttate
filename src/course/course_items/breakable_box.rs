use bevy_hanabi::prelude::*;

use super::*;

pub struct BreakableBoxPlugin;

impl Plugin for BreakableBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FireBreakEffect>()
            .insert_resource(BreakEffect(None))
            .add_systems(Startup,setup_break_effect)
            .add_systems(Update, (breakable_system, handle_break_effect));
    }
}

#[derive(Component)]
pub struct Breakable {
    required_speed: f32,
}
impl Breakable {
    pub fn new(required_speed: f32) -> Self {
        Self { required_speed }
    }
}

pub fn breakable_box_bundle(x: f32, y: f32, required_speed: f32) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        Breakable::new(required_speed),
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(ONE_BOX_SIZE / 2.0, ONE_BOX_SIZE / 2.0),
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.2),
            custom_size: Some(Vec2::new(ONE_BOX_SIZE, ONE_BOX_SIZE)),
            ..default()
        },
    )
}

#[derive(Message)]
struct FireBreakEffect(Vec3);

fn breakable_system(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    breakable_query: Query<(Entity, &Breakable)>,
    velocity_query: Query<&Velocity>,
    transform_query: Query<&Transform>,
    mut fire_break_effect: MessageWriter<FireBreakEffect>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (break_entity, other_entity, breakable) =
                if let Ok((entity, breakable)) = breakable_query.get(*e1) {
                    (entity, *e2, breakable)
                } else if let Ok((entity, breakable)) = breakable_query.get(*e2) {
                    (entity, *e1, breakable)
                } else {
                    continue;
                };

            let v1 = velocity_query.get(break_entity).ok();
            let v2 = velocity_query.get(other_entity).ok();

            let speed = match (v1, v2) {
                (Some(v1), Some(v2)) => (v1.linvel - v2.linvel).length(),
                (Some(v1), None) => v1.linvel.length(),
                (None, Some(v2)) => v2.linvel.length(),
                (None, None) => 0.0,
            };
            if speed >= breakable.required_speed {
                let position = transform_query
                    .get(break_entity)
                    .expect("break_entity don't have transform")
                    .translation;
                fire_break_effect.write(FireBreakEffect(position));
                commands.entity(break_entity).despawn();
            }
        }
    }
}

fn handle_break_effect(
    mut commands: Commands,
    effect: Res<BreakEffect>,
    mut fire_message: MessageReader<FireBreakEffect>,
) {
    for position in fire_message.read() {
        commands.spawn((
            ParticleEffect::new(effect.0.clone().expect("break effect never setuped")),
            Transform::from_translation(position.0),
        ));
    }
}

#[derive(Resource,Clone)]
struct BreakEffect(Option<Handle<EffectAsset>>);

fn setup_break_effect(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut break_effect_res: ResMut<BreakEffect>,
) {
    let handle = break_effect(&mut effects);
    break_effect_res.0 = Some(handle);
}

// break effect
fn break_effect(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut gradient = bevy_hanabi::Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 0., 0., 1.));
    gradient.add_key(1.0, Vec4::ZERO);

    let mut module = Module::default();

    let init_pos = SetPositionCircleModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(30.0),
        axis: module.lit(Vec3::Z),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        center: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::Z),
        speed: module.lit(100.),
    };

    let lifetime = module.lit(1.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        SpawnerSettings::once(100.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("berak_effect")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .render(ColorOverLifetimeModifier {
        gradient: gradient.into(),
        blend: ColorBlendMode::Overwrite,
        mask: ColorBlendMask::RGBA,
    });

    let effect_asset = effects.add(effect);
    effect_asset
}
