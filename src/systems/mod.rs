pub mod player;
pub mod resources;
pub mod animals;
pub mod camera;
pub mod ui;
pub mod tilemap;

pub use animals::*;
pub use camera::*;
pub use player::*;
pub use resources::*;
pub use tilemap::*;
pub use ui::*;


use bevy::prelude::*;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            TilemapPlugin,
            AnimalsPlugin,
            CameraPlugin,
            UiPlugin,
            ResourcesPlugin,
        ));
    }
}
