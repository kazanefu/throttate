use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use crate::DespawnWithTime;

pub fn spawn_effect(
    commands: &mut Commands,
    effect_handle: &Handle<EffectAsset>,
    position: Vec3,
    despawn_time: f32,
) {
    commands.spawn((
        ParticleEffect::new(effect_handle.clone()),
        Transform::from_translation(position),
        DespawnWithTime(despawn_time),
    ));
}
