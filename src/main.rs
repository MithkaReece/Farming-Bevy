mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use bevy::{ecs::schedule::ScheduleLabel, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};

use components::*;
use resources::*;
use systems::*;

fn main() {
    let mut schedule = Schedule::default();
    schedule.add_systems((give_seeds, setup_inventory.before(give_seeds)));

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
        .insert_resource(GroundTilemap {
            tiles: Vec::new(),
            width: 0,
            height: 0,
        })
        .insert_resource(ObjectTilemap {
            tiles: Vec::new(),
            width: 0,
            height: 0,
        })
        .add_systems(
            Startup,
            (setup_camera, setup_player, setup_tilemap, setup_inventory),
        )
        .add_systems(
            Update,
            (
                camera_follow,
                character_movement,
                plant_seed.before(hoe_ground),
                hoe_ground,
                spawn_sheep,
                sheep_lifetime,
                sheep_target_setter,
                sheep_movement,
                tile_hover,
                sync_tile_visual,
                give_seeds,
            ),
        )
        .run()
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_resolution(1000, 700, true));
}

fn give_seeds(mut query: Query<&mut Inventory>, input: Res<Input<KeyCode>>) {
    if !input.just_pressed(KeyCode::R) {
        return;
    }

    match query.get_single_mut() {
        Ok(mut inv) => {
            inv.add(ItemType::Seed(SeedType::Pumpkin));
            let count = inv.get_number(ItemType::Seed(SeedType::Pumpkin));
            println!("{:?}", count);
        }
        Err(_) => {
            // Handle the case where there is no or multiple Inventory components.
            // You can log an error or perform some other action here.
        }
    }
}
