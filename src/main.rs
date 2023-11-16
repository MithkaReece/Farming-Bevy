mod components;
mod config;
mod resources;
mod systems;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};

use components::*;
use config::*;
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
                        resolution: (1650.0 * 0.8, 1050.0 * 0.8).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .insert_resource(ScalingFactor {
            factor: 4.0,
            tile_size: 16,
        })
        .insert_resource(Money(100.0))
        .insert_resource(TilemapInfo {
            chunk_size: 16,
            dimensions: UVec3::new(1, 2, 3),
        })
        .insert_resource(TextureAtlasHandle {
            farm: Default::default(),
            plants: Default::default(),
        })
        .insert_resource(TilesData{
            tiles: Default::default(),
        })
        .add_plugins(SystemsPlugin)
        .run()
}