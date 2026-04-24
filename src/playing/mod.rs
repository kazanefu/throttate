use bevy::prelude::*;
mod main_camera;
mod player;
mod startup;
mod ui;
mod score;
pub use main_camera::*;
pub use player::*;
pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(startup::PlayingStartupPlugin)
            .add_plugins(main_camera::MainCameraPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(ui::PlayingUiPlugin);
    }
}
