pub mod camera_follow_system;
pub mod setup_camera_system;

use self::camera_follow_system::*;
use self::setup_camera_system::*;

use bevy::prelude::*;
use bevy_pixel_camera::PixelCameraPlugin;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, (camera_follow,));
        app.add_plugins(PixelCameraPlugin);
    }
}
