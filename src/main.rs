mod components;
mod resources;
mod systems;

use bevy::input::common_conditions::input_toggle_active;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::sprite::Anchor;
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
        .insert_resource(Money(100.0))
        .add_systems(Startup, (setup, setup_tilemap))
        .add_systems(
            Update,
            (
                camera_follow,
                character_movement,
                spawn_sheep,
                sheep_lifetime,
                sheep_target_setter,
                sheep_movement,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    //New camera
    commands.spawn(PixelCameraBundle::from_resolution(1000, 700, true));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("my-pixel-art-sprite.png"),
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        ..Default::default()
    });

    //Old camera
    // let mut camera = Camera2dBundle::default();

    // camera.projection.scaling_mode = ScalingMode::AutoMin {
    //     min_width: 256.0,
    //     min_height: 144.0,
    // };

    // commands.spawn(camera);

    // Load the sprite sheet image
    let texture_handle = asset_server.load("Thief_anim.png");
    // Create a TextureAtlas from the sprite sheet (with no padding and no offset)
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 30.0), 8, 5, None, None);
    // Add the TextureAtlas to the asset storage
    let atlas_handle = texture_atlases.add(texture_atlas);
    // Define the sprite for the specific frame you want to display
    let sprite_index = 0;

    // Spawn an entity with the selected sprite
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle,
            sprite: TextureAtlasSprite {
                index: sprite_index,
                ..Default::default()
            },
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..Default::default()
        },
        Player { speed: 500.0 },
    ));
}

pub struct Tile {
    pub sprite_index: usize,
}

#[derive(Component)]
pub struct Tilemap {
    pub atlas_handle: Handle<TextureAtlas>,
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

fn setup_tilemap(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // Load the sprite sheet image
    let texture_handle = asset_server.load("farm_tilemap.png");
    println!("{:?}", texture_handle);

    // Create a TextureAtlas from the sprite sheet (with no padding and no offset)
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 32, 16, None, None);

    // Add the TextureAtlas to the asset storage
    let atlas_handle = texture_atlases.add(texture_atlas);

    let map_width = 10;
    let map_height = 10;

    let mut tilemap = Tilemap {
        atlas_handle: atlas_handle.clone(),
        tiles: vec![],
        width: map_width,
        height: map_height,
    };
    let tile_scale = 4.0;
    // Add tiles to the tilemap
    for row in 0..map_height {
        for col in 0..map_width {
            let index = 32 * 5 + 1;

            let tile = Tile {
                sprite_index: index,
            };
            tilemap.tiles.push(tile);
            commands.spawn(SpriteSheetBundle {
                texture_atlas: atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: index,
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    (tile_scale * 16.0 * col as f32).round(),
                    (tile_scale * 16.0 * row as f32).round(),
                    -1.0,
                ) * Transform::from_scale(Vec3::splat(tile_scale)),
                ..Default::default()
            });
        }
    }
}
