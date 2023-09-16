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
    let map_chunk_width: usize = 20;
    let map_chunk_height: usize = 20;

    // Create ground tilemap
    ground_tilemap.num_chunks_width = map_chunk_width;
    ground_tilemap.num_chunks_height = map_chunk_height;

    // Create object tilemap
    object_tilemap.num_chunks_width = map_chunk_width;
    object_tilemap.num_chunks_height = map_chunk_height;

    for chunk_y in 0..map_chunk_height {
        for chunk_x in 0..map_chunk_width {
            let chunk_pos = Vec2::new(
                scaling_factor.get_full_factor() * (MAP_POS.x + (chunk_x * CHUNK_SIZE) as f32),
                scaling_factor.get_full_factor() * (MAP_POS.y + (chunk_y * CHUNK_SIZE) as f32),
            );

            let mut new_ground_chunk = Chunk {
                tiles: HashMap::new(),
                position: chunk_pos,
                is_loaded: false,
            };

            let mut new_object_chunk = Chunk {
                tiles: HashMap::new(),
                position: chunk_pos,
                is_loaded: false,
            };

            // Set ids of the tiles based on its positioj
            for row in 0..CHUNK_SIZE {
                for col in 0..CHUNK_SIZE {
                    let pos = Vec3::new(
                        chunk_pos.x + scaling_factor.get_full_factor() * col as f32,
                        chunk_pos.y + scaling_factor.get_full_factor() * row as f32,
                        0.0,
                    );
                    //println!("Pos: ({:?}, {:?})", pos.x as usize, pos.y as usize);
                    // Insert tile into ground chunk
                    new_ground_chunk.tiles.insert(
                        (pos.x as usize, pos.y as usize),
                        Tile {
                            position: pos,
                            tile_type: TileType::Grass,
                            visible: true,
                            index_offset: 0,
                        },
                    );
                    // Insert tile into object chunk
                    new_object_chunk.tiles.insert(
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
            ground_tilemap.tiles.insert(
                (chunk_pos.x as usize, chunk_pos.y as usize),
                new_ground_chunk,
            );
            object_tilemap.tiles.insert(
                (chunk_pos.x as usize, chunk_pos.y as usize),
                new_object_chunk,
            );
            //println!("ADD chunk: ({:?},{:?})", chunk_pos.x, chunk_pos.y);
        }
    }
}
