use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use super::utils::spawn_effect;

#[derive(Message)]
pub struct FireDeathEffect(pub Vec3);

#[derive(Resource, Clone)]
pub struct DeathEffect(pub Option<Handle<EffectAsset>>);

pub fn handle_death_effect(
    mut commands: Commands,
    effect: Res<DeathEffect>,
    mut fire_message: MessageReader<FireDeathEffect>,
) {
    let Some(handle) = &effect.0 else {
        return;
    };
    for position in fire_message.read() {
        spawn_effect(&mut commands, handle, position.0, 1.0);
    }
}

pub fn setup_death_effect(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut death_effect_res: ResMut<DeathEffect>,
) {
    let handle = death_effect_asset(&mut effects);
    death_effect_res.0 = Some(handle);
}

fn death_effect_asset(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut gradient: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.0, 0.0, 1.));
    gradient.add_key(1.0, Vec4::new(1.0, 1.0, 0.0, 0.01));

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
        speed: module.lit(1000.),
    };

    let lifetime = module.lit(1.);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let effect = EffectAsset::new(
        32768,
        SpawnerSettings::once(10.0.into()),
        module,
    )
    .with_name("death_effect")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .render(ColorOverLifetimeModifier {
        gradient,
        blend: ColorBlendMode::Modulate,
        mask: ColorBlendMask::RGBA,
    })
    .render(SizeOverLifetimeModifier {
        gradient: bevy_hanabi::Gradient::from_keys(vec![
            (0.0, Vec3::splat(20.0)),
            (1.0, Vec3::splat(0.0)),
        ]),
        screen_space_size: false,
    });

    effects.add(effect)
}
