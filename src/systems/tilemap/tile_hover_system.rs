use bevy::prelude::*;

use crate::{
    components::{Player, Tilemap},
    config::layer_enum::TilemapLayer,
    resources::ScalingFactor,
};

pub fn tile_hover(
    player: Query<&Player>,
    mut tile_entities: Query<&mut TextureAtlasSprite>,
    scaling_factor: ResMut<ScalingFactor>,
    tilemap: Query<&Tilemap>,
) {
    let tilemap = tilemap.single();
    let chunk_size = tilemap.chunk_size as u32;

    //Find chunk and tile position given a real-world coordinate
    let (chunk_pos, tile_pos) = tilemap.real_to_chunk_and_tile(
        &player.single().looking_location,
        scaling_factor.get_full_factor(),
    );

    

    for (chunk_x, row) in tilemap.chunks.iter().enumerate() {
        for (chunk_y, col) in row.iter().enumerate() {
            //TODO: Should just be able to take out the ground layer instead of iterating
            for (chunk_z, chunk) in col.iter().enumerate() {
                // Only consider ground layer
                if chunk_z != TilemapLayer::Ground as usize {
                    continue;
                }

                if chunk_x as u32 == chunk_pos.x && chunk_y as u32 == chunk_pos.y {
                    for y in 0..chunk_size as u32 {
                        for x in 0..chunk_size as u32 {
                            let entity = match chunk.get_tile_entity(&UVec2::new(x, y)) {
                                Some(entity) => entity,
                                None => continue,
                            };
                            // Highlight tile if matches highlighted position
                            if let Ok(mut sprite) = tile_entities.get_mut(*entity) {
                                if x == tile_pos.x && y == tile_pos.y {
                                    sprite.color = Color::Rgba {
                                        red: 1.0,
                                        green: 0.9,
                                        blue: 0.9,
                                        alpha: 1.0,
                                    };
                                } else {
                                    sprite.color = Color::WHITE;
                                }
                            }
                        }
                    }
                } else {
                    for y in 0..chunk_size as u32 {
                        for x in 0..chunk_size as u32 {
                            let entity = match chunk.get_tile_entity(&UVec2::new(x, y)) {
                                Some(entity) => entity,
                                None => continue,
                            };
                            if let Ok(mut sprite) = tile_entities.get_mut(*entity) {
                                sprite.color = Color::WHITE;
                            }
                        }
                    }
                }
            }
        }
    }
}
