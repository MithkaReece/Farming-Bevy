use bevy::prelude::*;

use crate::{
    components::{Tile, Tilemap},
    resources::TilemapInfo,
};

pub fn setup_tilemap(
    mut commands: Commands,
    tilemap_info: Res<TilemapInfo>,
) {
    let mut tilemap = Tilemap::new(tilemap_info.dimensions, tilemap_info.chunk_size);

    // Generate ground 
    for chunk_y in 0..tilemap.dimensions.y as u32 {
        for chunk_x in 0..tilemap.dimensions.x as u32 {
            for row in 0..tilemap_info.chunk_size as u32 {
                for col in 0..tilemap_info.chunk_size as u32 {
                    match tilemap.set_tile(
                    &UVec3::new(chunk_x, chunk_y, 0),
                    &UVec2::new(col, row),
                    Tile::Grass) {
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
