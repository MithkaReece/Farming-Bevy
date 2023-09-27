use bevy::prelude::*;
use bevy_pixel_camera::PixelCameraBundle;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_resolution(
        (1650.0 * 0.8) as i32,
        (1050.0 * 0.8) as i32,
        true,
    ));
}
