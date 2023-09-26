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
    let mut schedule = Schedule::default();
    schedule.add_systems((give_seeds, setup_inventory.before(give_seeds)));

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
        .add_plugins(PixelCameraPlugin)
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
            dimensions: UVec3::new(1, 2, 2),
        })
        .insert_resource(TextureAtlasHandle {
            farm: Default::default(),
            plants: Default::default(),
        })
        .add_systems(
            Startup,
            (
                setup_textures,
                setup_camera,
                setup_player,
                setup_tilemap,
                setup_inventory,
                // spawn_inventory_ui,
            ),
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
                animal_target_setter,
                sheep_movement,
                tile_hover,
                sync_tile_visual,
                give_seeds,
                plant_growth,
                harvest_plant,
                chunk_loading,
                animal_ai,
                animal_stats,
                // ui_inventory,
            ),
        )
        .run()
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_resolution(
        (1650.0 * 0.8) as i32,
        (1050.0 * 0.8) as i32,
        true,
    ));
}

fn give_seeds(mut query: Query<&mut Inventory>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::R) {
        let mut inv = query.single_mut();
        inv.add(ItemType::Seed(SeedType::Pumpkin));
        let count = inv.get_number(ItemType::Seed(SeedType::Pumpkin));
        println!("{:?}", count);
    }
}

// pub fn spawn_inventory_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
//     //let inventory_ui_entity = build_inventory_ui(&mut commands, asset_server);
//     // Create a UI node with text
//     commands.spawn(TextBundle {
//         text: Text {
//             sections: vec![TextSection {
//                 value: "Hello, Bevy!".to_string(),
//                 style: TextStyle {
//                     font: Default::default(),
//                     font_size: 40.0,
//                     color: Color::WHITE,
//                 },
//             }],
//             ..Default::default()
//         },
//         ..Default::default()
//     });
// }

// pub fn build_inventory_ui(commands: &mut Commands, asset_server: Res<AssetServer>) -> Entity {
//     // Load the item texture and store it with a handle
//     //let item_texture_handle = asset_server.load("item_texture.png");

//     let inventory_ui_entity = commands
//         .spawn(NodeBundle {
//             style: Style {
//                 display: Display::Flex,
//                 width: Val::Percent(100.0),
//                 height: Val::Percent(100.0),
//                 align_items: AlignItems::Center,
//                 justify_content: JustifyContent::Center,
//                 ..default()
//             },
//             background_color: Color::Rgba {
//                 red: 255.0,
//                 green: 0.0,
//                 blue: 0.0,
//                 alpha: 100.0,
//             }
//             .into(),
//             ..default()
//         })
//         .with_children(|builder| {
//             let rows = 4;
//             let columns = 10;

//             //let square_size = 100.0 / f32::max(rows as f32, columns as f32) as f32;
//             let ratio = columns as f32 / rows as f32;
//             // Grid
//             builder
//                 .spawn(NodeBundle {
//                     style: Style {
//                         width: Val::Percent(80.0),
//                         aspect_ratio: Some(ratio),
//                         display: Display::Grid,
//                         padding: UiRect::all(Val::Px(5.0)),
//                         // 4 Columns
//                         grid_template_columns: RepeatedGridTrack::flex(columns, 1.0),
//                         grid_template_rows: RepeatedGridTrack::flex(rows, 1.0),
//                         row_gap: Val::Px(2.0),
//                         column_gap: Val::Px(2.0),
//                         ..default()
//                     },
//                     background_color: BackgroundColor(Color::DARK_GRAY),
//                     ..default()
//                 })
//                 .with_children(|builder| {
//                     for _ in 0..rows * columns {
//                         item_rect(builder, Color::ORANGE);
//                     }
//                 });
//         })
//         .id();

//     inventory_ui_entity
// }

// #[derive(Component)]
// struct InventoryCell {
//     pub item_name: String,
// }

// fn item_rect(builder: &mut ChildBuilder, color: Color) {
//     builder
//         .spawn((
//             NodeBundle {
//                 style: Style {
//                     display: Display::Grid,
//                     // padding: UiRect::all(Val::Px(3.0)),
//                     ..default()
//                 },
//                 background_color: BackgroundColor(Color::BLACK),
//                 ..default()
//             },
//             InventoryCell {
//                 item_name: "Cell".to_string(),
//             },
//         ))
//         .with_children(|builder| {
//             builder
//                 .spawn(NodeBundle {
//                     background_color: BackgroundColor(color),
//                     ..default()
//                 })
//                 .with_children(|builder| {
//                     builder.spawn(TextBundle {
//                         text: Text {
//                             sections: vec![TextSection {
//                                 value: "fail".to_string(),
//                                 style: TextStyle {
//                                     font: Default::default(),
//                                     font_size: 40.0,
//                                     color: Color::WHITE,
//                                 },
//                             }],
//                             ..default()
//                         },
//                         ..default()
//                     });
//                 });
//         });
// }

// //System
// fn ui_inventory(mut query: Query<&mut Text, With<InventoryCell>>) {
//     for (mut text) in &mut query {
//         println!("Test");
//         text.sections[0].value = "Test".to_string();
//         // Turn the text purple
//         text.sections[0].style.color = Color::Rgba {
//             red: 1.0,
//             green: 0.0,
//             blue: 1.0,
//             alpha: 1.0,
//         };
//     }
// }
