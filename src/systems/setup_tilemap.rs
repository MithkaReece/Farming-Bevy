use bevy::prelude::*;

use crate::{
    components::{Tile, TileType, Tileref, item_component::SeedType},
    resources::{
        tilemap_resource::{GroundTilemap, ObjectTilemap},
        ScalingFactor,
    },
};

pub fn setup_tilemap(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut ground_tilemap: ResMut<GroundTilemap>,
    mut object_tilemap: ResMut<ObjectTilemap>,
    scaling_factor: ResMut<ScalingFactor>,
) {
    // Load ground spritesheet
    let ground_texture_handle = asset_server.load("farm_tilemap.png");
    let ground_texture_atlas = TextureAtlas::from_grid(
        ground_texture_handle,
        Vec2::new(16.0, 16.0),
        32,
        16,
        None,
        None,
    );
    let ground_atlas_handle = texture_atlases.add(ground_texture_atlas);

    // Load ground spritesheet
    let plant_texture_handle = asset_server.load("plants.png");
    let plant_texture_atlas = TextureAtlas::from_grid(
        plant_texture_handle,
        Vec2::new(16.0, 16.0),
        5,
        6,
        None,
        None,
    );
    let plant_atlas_handle = texture_atlases.add(plant_texture_atlas);

    let map_width = 10;
    let map_height = 10;
    ground_tilemap.width = map_width;
    ground_tilemap.height = map_height;
    object_tilemap.width = map_width;
    object_tilemap.height = map_height;

    let mut tile_id = 0;
    // Add tiles to the tilemap
    for row in 0..map_height {
        for col in 0..map_width {
            let index = 32 * 5 + 1;
            // Figure out position
            let pos = Vec2::new(
                (scaling_factor.factor * scaling_factor.pixel_factor as f32 * col as f32).round(),
                (scaling_factor.factor * scaling_factor.pixel_factor as f32 * row as f32).round(),
            );
            // Create ground tile
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: ground_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
                        index,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(pos.x, pos.y, -2.0)
                        * Transform::from_scale(Vec3::splat(scaling_factor.factor)),
                    ..Default::default()
                },
                Tile {
                    unique_id: tile_id,
                    tile_type: TileType::Grass,
                    visible: true,
                },
            ));
            // Track tile in tilemap
            ground_tilemap.tiles.push(Tileref {
                position: pos,
                unique_id: tile_id,
            });
            // Increment identifer
            tile_id += 1;

            let visible = rand::random::<f32>() > 0.5;

            // Create plant tile
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: plant_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
                        index: 1,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(pos.x, pos.y, -1.0)
                        * Transform::from_scale(Vec3::splat(scaling_factor.factor)),
                    ..Default::default()
                },
                Tile {
                    unique_id: tile_id,
                    tile_type: TileType::Seed(SeedType::Pumpkin),
                    visible,
                },
            ));
            // Track tile in tilemap
            object_tilemap.tiles.push(Tileref {
                position: pos,
                unique_id: tile_id,
            });
            // Increment identifer
            tile_id += 1;
        }
    }
}
