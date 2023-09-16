use bevy::{prelude::*, utils::HashMap};

use crate::{
    components::{
        chunk_component::{Chunk, CHUNK_SIZE},
        item_component::SeedType,
        Tile, TileType, Tileref,
    },
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

    let map_pos = Vec2::new(0.0, 0.0);

    /*Generate chunks */
    let map_chunk_width: usize = 20;
    let map_chunk_height: usize = 20;

    // Create ground tilemap
    ground_tilemap.num_chunks_width = map_chunk_width;
    ground_tilemap.num_chunks_height = map_chunk_height;

    for chunk_y in 0..map_chunk_height {
        for chunk_x in 0..map_chunk_width {
            let mut new_chunk = Chunk {
                tiles: [[Tile {
                    position: Vec3::new(0.0, 0.0, 0.0),
                    tile_type: TileType::Grass,
                    visible: true,
                    index_offset: 0,
                }; CHUNK_SIZE]; CHUNK_SIZE],
                position: Vec2::new(
                    scaling_factor.get_full_factor() *(map_pos.x + (chunk_x * CHUNK_SIZE) as f32),
                    scaling_factor.get_full_factor() *(map_pos.y + (chunk_y * CHUNK_SIZE) as f32),
                ),
                is_loaded: false,
            };

            // Set ids of the tiles based on its positioj
            for row in 0..CHUNK_SIZE {
                for col in 0..CHUNK_SIZE {
                    // Translate to map pos
                    // Translate to correct chunk
                    // Translate to row and col in chunk
                    // Scale up

                    let pos = Vec3::new(
                        scaling_factor.get_full_factor() * (map_pos.x
                            + (chunk_x * CHUNK_SIZE + col) as f32),
                        scaling_factor.get_full_factor() * (map_pos.y
                            + (chunk_y * CHUNK_SIZE + row) as f32),
                        0.0,
                    );
                    new_chunk.tiles[col][row].position = pos;
                }
            }

            ground_tilemap.tiles.insert((chunk_x, chunk_y), new_chunk);
        }
    }

    // Create object tilemap
    object_tilemap.num_chunks_width = map_chunk_width;
    object_tilemap.num_chunks_height = map_chunk_height;

    for chunk_y in 0..map_chunk_height {
        for chunk_x in 0..map_chunk_width {
            let mut new_chunk = Chunk {
                tiles: [[Tile {
                    position: Vec3::new(0.0, 0.0, 0.0),
                    tile_type: TileType::None,
                    visible: false,
                    index_offset: 0,
                }; CHUNK_SIZE]; CHUNK_SIZE],
                position: Vec2::new(
                    map_pos.x + (chunk_x * CHUNK_SIZE) as f32,
                    map_pos.y + (chunk_y * CHUNK_SIZE) as f32,
                ),
                is_loaded: false,
            };

            // Set ids of the tiles based on its positioj
            for row in 0..CHUNK_SIZE {
                for col in 0..CHUNK_SIZE {
                    let pos = Vec3::new(
                        map_pos.x + (chunk_x * CHUNK_SIZE + col) as f32,
                        map_pos.y + (chunk_y * CHUNK_SIZE + row) as f32,
                        1.0,
                    );
                    new_chunk.tiles[col][row].position = pos;
                }
            }

            object_tilemap.tiles.insert((chunk_x, chunk_y), new_chunk);
        }
    }

    // let map_width = 30;
    // let map_height = 30;
    // ground_tilemap.width = map_width;
    // ground_tilemap.height = map_height;
    // object_tilemap.width = map_width;
    // object_tilemap.height = map_height;

    // let mut tile_id = 0;
    // // Add tiles to the tilemap
    // for row in 0..map_height {
    //     for col in 0..map_width {
    //         let index = 32 * 5 + 1;
    //         // Figure out position
    //         let pos = Vec2::new(
    //             (scaling_factor.factor * scaling_factor.pixel_factor as f32 * col as f32).round(),
    //             (scaling_factor.factor * scaling_factor.pixel_factor as f32 * row as f32).round(),
    //         );
    //         // Create ground tile
    //         let entity = commands.spawn((
    //             SpriteSheetBundle {
    //                 texture_atlas: ground_atlas_handle.clone(),
    //                 sprite: TextureAtlasSprite {
    //                     index,
    //                     ..Default::default()
    //                 },
    //                 transform: Transform::from_xyz(pos.x, pos.y, -2.0)
    //                     * Transform::from_scale(Vec3::splat(scaling_factor.factor)),
    //                 ..Default::default()
    //             },
    //             Tile {
    //                 unique_id: tile_id,
    //                 tile_type: TileType::Grass,
    //                 visible: true,
    //                 index_offset: 0,
    //             },
    //         )).id();
    //         // Track tile in tilemap
    //         ground_tilemap.tiles.push(Tileref {
    //             position: pos,
    //             unique_id: tile_id,
    //         });
    //         // Increment identifer
    //         tile_id += 1;

    //         // Create plant tile
    //         commands.spawn((
    //             SpriteSheetBundle {
    //                 texture_atlas: plant_atlas_handle.clone(),
    //                 sprite: TextureAtlasSprite {
    //                     index: 1,
    //                     ..Default::default()
    //                 },
    //                 transform: Transform::from_xyz(pos.x, pos.y, -1.0)
    //                     * Transform::from_scale(Vec3::splat(scaling_factor.factor)),
    //                 ..Default::default()
    //             },
    //             Tile {
    //                 unique_id: tile_id,
    //                 tile_type: TileType::None,
    //                 visible: false,
    //                 index_offset: 0,
    //             },
    //         ));
    //         // Track tile in tilemap
    //         object_tilemap.tiles.push(Tileref {
    //             position: pos,
    //             unique_id: tile_id,
    //         });
    //         // Increment identifer
    //         tile_id += 1;
    //     }
    // }
}

