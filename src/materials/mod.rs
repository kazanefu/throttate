use bevy::prelude::*;
mod death_vignette;
mod meteor;

pub use meteor::MeteorMaterial;

pub struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(death_vignette::DeathEffectPlugin)
            .add_plugins(meteor::MeteorMaterialPlugin);
    }
}
