use bevy::prelude::*;

use crate::{
    components::{Tile, Tilemap, GroundType},
    resources::{TilemapInfo, tile_info_resource::TilesData},
};

use noise::{NoiseFn, Perlin};

/**
Biomes 
Default (grass with ponds)
*/

pub fn setup_tilemap(
    mut commands: Commands,
    tilemap_info: Res<TilemapInfo>,
) {
    let mut tilemap = Tilemap::new(tilemap_info.dimensions, tilemap_info.chunk_size);

    //TODO: In future load data about world generation in and use that
    
    let perlin = Perlin::new(1); // Output range (-1,1)
    let tilemap_tile_width = tilemap_info.dimensions.x as f64 * tilemap_info.chunk_size as f64;
    let tilemap_tile_height = tilemap_info.dimensions.y as f64 * tilemap_info.chunk_size as f64;

    // Generate ground 
    for chunk_y in 0..tilemap.dimensions.y as u32 {
        for chunk_x in 0..tilemap.dimensions.x as u32 {
            for row in 0..tilemap_info.chunk_size as u32 {
                for col in 0..tilemap_info.chunk_size as u32 {
                    //Determine tile type
                    let val = perlin.get([
                        (chunk_x as f64 * tilemap_info.chunk_size as f64 + col as f64) / tilemap_tile_width, 
                        (chunk_y as f64 * tilemap_info.chunk_size as f64 + row as f64) / tilemap_tile_height,
                    ]);

                    //println!("{val}");
                    let tile_type = if val > 0.0 { Tile::Ground(GroundType::Grass) } else { Tile::Ground(GroundType::Water) };


                    match tilemap.set_tile(
                        &UVec3::new(chunk_x, chunk_y, 0),
                        &UVec2::new(col, row),
                        tile_type
                        ) {
                            Err(e) => println!("{e}"),
                            Ok(()) => {},
                        };
                }
            }
        }
    }

    commands.spawn(
        tilemap 
    );
}
