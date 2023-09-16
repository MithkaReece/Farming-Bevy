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

pub const MAP_POS: Vec2 = Vec2::new(0.0, 0.0);

pub fn setup_tilemap(
    mut ground_tilemap: ResMut<GroundTilemap>,
    mut object_tilemap: ResMut<ObjectTilemap>,
    scaling_factor: ResMut<ScalingFactor>,
) {
    /*Generate chunks */
    let map_chunk_width: usize = 1;
    let map_chunk_height: usize = 1;

    // Create ground tilemap
    ground_tilemap.num_chunks_width = map_chunk_width;
    ground_tilemap.num_chunks_height = map_chunk_height;

    for chunk_y in 0..map_chunk_height {
        for chunk_x in 0..map_chunk_width {
            let chunk_pos = Vec2::new(
                scaling_factor.get_full_factor() * (MAP_POS.x + (chunk_x * CHUNK_SIZE) as f32),
                scaling_factor.get_full_factor() * (MAP_POS.y + (chunk_y * CHUNK_SIZE) as f32),
            );

            let mut new_chunk = Chunk {
                tiles: HashMap::new(),
                position: chunk_pos,
                is_loaded: false,
            };

            // Set ids of the tiles based on its positioj
            for row in 0..CHUNK_SIZE {
                for col in 0..CHUNK_SIZE {
                    // Translate to row and col in chunk
                    // Scale up

                    let pos = Vec3::new(
                        scaling_factor.get_full_factor() * (chunk_pos.x + col as f32),
                        scaling_factor.get_full_factor() * (chunk_pos.y + row as f32),
                        0.0,
                    );
                    //println!("Pos: ({:?}, {:?})", pos.x as usize, pos.y as usize);
                    new_chunk.tiles.insert(
                        (pos.x as usize, pos.y as usize),
                        Tile {
                            position: pos,
                            tile_type: TileType::Grass,
                            visible: true,
                            index_offset: 0,
                        },
                    );
                }
            }

            ground_tilemap
                .tiles
                .insert((chunk_pos.x as usize, chunk_pos.y as usize), new_chunk);
        }
    }

    // Create object tilemap
    object_tilemap.num_chunks_width = map_chunk_width;
    object_tilemap.num_chunks_height = map_chunk_height;

    for chunk_y in 0..map_chunk_height {
        for chunk_x in 0..map_chunk_width {
            let chunk_pos = Vec2::new(
                scaling_factor.get_full_factor() * (MAP_POS.x + (chunk_x * CHUNK_SIZE) as f32),
                scaling_factor.get_full_factor() * (MAP_POS.y + (chunk_y * CHUNK_SIZE) as f32),
            );

            let mut new_chunk = Chunk {
                tiles: HashMap::new(),
                position: chunk_pos,
                is_loaded: false,
            };

            // Set ids of the tiles based on its positioj
            for row in 0..CHUNK_SIZE {
                for col in 0..CHUNK_SIZE {
                    let pos = Vec3::new(
                        scaling_factor.get_full_factor() * (chunk_pos.x + col as f32),
                        scaling_factor.get_full_factor() * (chunk_pos.y + row as f32),
                        1.0,
                    );
                    new_chunk.tiles.insert(
                        (pos.x as usize, pos.y as usize),
                        Tile {
                            position: pos,
                            tile_type: TileType::None,
                            visible: false,
                            index_offset: 0,
                        },
                    );
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
