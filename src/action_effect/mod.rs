use bevy::prelude::*;

pub mod checkpoint;
pub mod death;
pub mod utils;

pub use checkpoint::{CheckPointEffect, FireCheckPointEffect};
pub use death::{DeathEffect, FireDeathEffect};

pub struct ActionEffectPlugin;

impl Plugin for ActionEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FireDeathEffect>()
            .add_message::<FireCheckPointEffect>()
            .insert_resource(DeathEffect(None))
            .insert_resource(CheckPointEffect(None))
            .add_systems(
                Startup,
                (
                    death::setup_death_effect,
                    checkpoint::setup_checkpoint_effect,
                ),
            )
            .add_systems(
                Update,
                (
                    death::handle_death_effect,
                    checkpoint::handle_checkpoint_effect,
                ),
            );
    }
}