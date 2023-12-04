use bevy::prelude::*;

use crate::{
    components::{GroundType, Tile, TileType, Tilemap},
    config::layer_enum::TilemapLayer,
    resources::TilemapInfo,
};

use noise::{NoiseFn, Perlin};

/**
Biomes
Default (grass with ponds)
*/

pub fn setup_tilemap(mut commands: Commands, tilemap_info: Res<TilemapInfo>) {
    let mut tilemap = Tilemap::new(tilemap_info.dimensions, tilemap_info.chunk_size);

    //TODO: In future load data about world generation in and use that

    let perlin = Perlin::new(1); // Output range (-1,1)
    let tilemap_tile_width = tilemap_info.dimensions.x as f64 * tilemap_info.chunk_size as f64;
    let tilemap_tile_height = tilemap_info.dimensions.y as f64 * tilemap_info.chunk_size as f64;

    for x in 0..tilemap.dimensions.x * tilemap.chunk_size as u32 {
        for y in 0..tilemap.dimensions.y * tilemap.chunk_size as u32 {
            let val = perlin.get([
                x as f64 / tilemap_tile_width,
                y as f64 / tilemap_tile_height,
            ]);

            let tile = if val > 0.0 {
                Tile {
                    tile_type: TileType::Ground(GroundType::Grass),
                    has_collision: false,
                }
            } else {
                Tile {
                    tile_type: TileType::Ground(GroundType::Water),
                    has_collision: true,
                }
            };

            match tilemap.set_tile(&UVec3::new(x, y, TilemapLayer::Ground as u32), tile) {
                Err(e) => println!("{e}"),
                Ok(()) => {}
            };
        }
    }

    commands.spawn(tilemap);
}
