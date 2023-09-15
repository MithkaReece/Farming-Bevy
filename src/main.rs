mod components;
mod resources;
mod systems;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};

use components::*;
use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Project Game".into(),
                        resolution: (1000.0, 700.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(PixelCameraPlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .insert_resource(ScalingFactor {
            factor: 4.0,
            pixel_factor: 16,
        })
        .insert_resource(Money(100.0))
        .insert_resource(Tilemap {
            tiles: Vec::new(),
            width: 0,
            height: 0,
        })
        .add_systems(Startup, (setup_camera, setup_player, setup_tilemap))
        .add_systems(
            Update,
            (
                camera_follow,
                character_movement,
                hoe_ground,
                spawn_sheep,
                sheep_lifetime,
                sheep_target_setter,
                sheep_movement,
                tile_hover,
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_resolution(1000, 700, true));
}
