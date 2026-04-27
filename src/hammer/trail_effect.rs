use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::hammer::definition::Hammer;

#[derive(Component)]
struct TrailEffect;
#[derive(Component)]
pub struct HasTrailEffect;

pub fn trail_effect_bundle(effects: &mut Assets<EffectAsset>) -> impl Bundle {
    let mut gradient: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.0, 0.0, 1.0, 1.));
    gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.01));

    let mut module = Module::default();

    let init_pos = SetPositionCircleModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(15.0),
        axis: module.lit(Vec3::Z),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        center: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::Z),
        speed: module.lit(0.0),
    };

    let lifetime = module.lit(1.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        SpawnerSettings::rate(100.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("berak_effect")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .render(ColorOverLifetimeModifier {
        gradient, // rust-analyzer says `expected Gradient, found Gradient<Vec4>` but that is just a bug of rust-analyzer.  this code works!
        blend: ColorBlendMode::Modulate,
        mask: ColorBlendMask::RGBA,
    })
    .render(SizeOverLifetimeModifier {
        // rust-analyzer says `expected Gradient, found Gradient<Vec3>` but that is just a bug of rust-analyzer. this code works!
        gradient: bevy_hanabi::Gradient::from_keys(vec![
            (0.0, Vec3::splat(5.0)),
            (1.0, Vec3::splat(0.0)),
        ]),
        screen_space_size: false,
    });

    let effect_handle = effects.add(effect);
    (
        TrailEffect,
        ParticleEffect::new(effect_handle),
        Transform::from_translation(-Vec3::Z),
    )
}

pub fn attach_trail_effect(
    mut commands: Commands,
    hammer_query: Query<Entity, (With<Hammer>, Without<HasTrailEffect>)>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    for hammer_entity in &hammer_query {
        let trail_effect = commands
            .spawn(trail_effect_bundle(&mut effects))
            .id();
        commands.entity(hammer_entity).add_child(trail_effect).insert(HasTrailEffect);
    }
}
