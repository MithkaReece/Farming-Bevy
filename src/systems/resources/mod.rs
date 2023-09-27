pub mod setup_textures;

use self::setup_textures::*;

use bevy::prelude::*;

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_textures);
    }
}
