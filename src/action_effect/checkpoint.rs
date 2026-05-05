use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use super::utils::spawn_effect;

#[derive(Message)]
pub struct FireCheckPointEffect(pub Vec3);

#[derive(Resource, Clone)]
pub struct CheckPointEffect(pub Option<Handle<EffectAsset>>);

pub fn handle_checkpoint_effect(
    mut commands: Commands,
    effect: Res<CheckPointEffect>,
    mut fire_message: MessageReader<FireCheckPointEffect>,
    mut count: Local<i32>,
) {
    let Some(handle) = &effect.0 else {
        return;
    };
    for position in fire_message.read() {
        spawn_effect(&mut commands, handle, position.0, 5.0);
        *count += 1;
        println!("reach checkpoint {}", *count);
    }
}

pub fn setup_checkpoint_effect(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut checkpoint_effect_res: ResMut<CheckPointEffect>,
) {
    let handle = checkpoint_effect_asset(&mut effects);
    checkpoint_effect_res.0 = Some(handle);
}

pub fn checkpoint_effect_asset(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut gradient: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.0, 0.4, 0.8, 1.));
    gradient.add_key(1.0, Vec4::new(0.0, 1.0, 1.0, 0.3));

    let mut module = Module::default();

    let init_pos = SetPositionCircleModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(30.0),
        axis: module.lit(Vec3::Z),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocityCircleModifier {
        center: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::Z),
        speed: module.lit(30.0),
    };

    let lifetime = module.lit(3.);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let effect = EffectAsset::new(
        32768,
        SpawnerSettings::rate(10.0.into()).with_spawn_duration(1.0.into()).with_cycle_count(1),
        module,
    )
    .with_name("checkpoint_effect")
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
            (0.0, Vec3::splat(15.0)),
            (1.0, Vec3::splat(0.0)),
        ]),
        screen_space_size: false,
    });
    
    effects.add(effect)
}
