use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};
use bevy_rapier2d::prelude::*;

pub struct MeteorMaterialPlugin;

impl Plugin for MeteorMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<MeteorMaterial>::default())
            .add_systems(Update, update_meteor_material);
    }
}

#[derive(ShaderType, Clone, Debug, Copy, Default)]
struct MeteorUniform {
    velocity_dir: Vec2,
    speed_squared: f32,
    world_position: Vec2,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct MeteorMaterial {
    #[uniform(0)]
    params: MeteorUniform,
}

impl Material2d for MeteorMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/meteor.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

fn update_meteor_material(
    mut materials: ResMut<Assets<MeteorMaterial>>,

    query: Query<(&MeshMaterial2d<MeteorMaterial>, &Velocity, &GlobalTransform)>,
) {
    for (material_handle, velocity, global_transform) in &query {
        if let Some(material) = materials.get_mut(material_handle) {
            let linvel = velocity.linvel;

            let speed_squared = linvel.length_squared();

            material.params.speed_squared = speed_squared;

            material.params.world_position = global_transform.translation().truncate();

            material.params.velocity_dir = velocity.linvel.normalize_or(Vec2::X);
        }
    }
}
