use bevy::prelude::*;

use crate::{
    components::{Player, Tilemap},
    config::layer_enum::Layer,
    resources::ScalingFactor,
};

pub fn tile_hover(
    player: Query<&Transform, With<Player>>,
    mut tile_entities: Query<&mut TextureAtlasSprite>,
    scaling_factor: ResMut<ScalingFactor>,
    tilemap: Query<&Tilemap>,
) {
    let full_scaling_factor = scaling_factor.get_full_factor();
    let player_transform = player.single();
    let player_position = Vec2::new(
        player_transform.translation.x + full_scaling_factor / 2.0,
        player_transform.translation.y + full_scaling_factor / 2.0,
    );

    let tilemap = tilemap.single();
    let chunk_size = tilemap.chunk_size as u32;

    let (chunk_pos, tile_pos) = tilemap.from_pos_no_layer(&player_position, scaling_factor.get_full_factor());

    for (chunk_x, row) in tilemap.chunks.iter().enumerate() {
        for (chunk_y, col) in row.iter().enumerate() {
            for (chunk_z, chunk) in col.iter().enumerate() {
                if chunk_z != Layer::Ground as usize {
                    continue;
                }

                if chunk_x as u32 == chunk_pos.x && chunk_y as u32 == chunk_pos.y {
                    for y in 0..chunk_size as u32 {
                        for x in 0..chunk_size as u32 {
                            let entity = match chunk.get_tile_entity(&UVec2::new(x, y)) {
                                Some(entity) => entity,
                                None => continue,
                            };
                            if let Ok((mut sprite)) = tile_entities.get_mut(*entity) {
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
                    for _ in 0..chunk_size as u32 {
                        for _ in 0..chunk_size as u32 {
                            let entity = match chunk.get_tile_entity(&tile_pos) {
                                Some(entity) => entity,
                                None => continue,
                            };
                            if let Ok((mut sprite)) = tile_entities.get_mut(*entity) {
                                sprite.color = Color::WHITE;
                            }
                        }
                    }
                }
            }
        }
    }
}
