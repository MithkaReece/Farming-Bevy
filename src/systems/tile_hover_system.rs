use bevy::prelude::*;

use crate::{
    components::{chunk_component::EntityChunkMapping, Player, Tile},
    resources::{tilemap_resource::GroundTilemap, ScalingFactor},
    //systems::get_ground_tile_id,
};

use super::{get_ground_tile, pos_to_tile_pos};

pub fn tile_hover(
    player: Query<&Transform, With<Player>>,
    mut tiles: Query<&mut TextureAtlasSprite>,
    ground_tilemap: ResMut<GroundTilemap>,
    scaling_factor: ResMut<ScalingFactor>,
    entity_chunk_map: Res<EntityChunkMapping>,
) {
    let full_scaling_factor = scaling_factor.get_full_factor();
    let player_transform = player.single();
    let player_position = Vec2::new(
        player_transform.translation.x + full_scaling_factor / 2.0,
        player_transform.translation.y + full_scaling_factor / 2.0,
    );
    let player_tile_pos = pos_to_tile_pos(player_position, full_scaling_factor);

    // Loop through all loaded chunks
    for ((chunk_x, chunk_y), chunk) in &ground_tilemap.tiles {
        // If a chunk is loaded
        if !chunk.is_loaded {
            continue;
        }
        // Get entities for each key (position)
        let entities_option = entity_chunk_map.mapping.get(&(*chunk_x, *chunk_y));
        if entities_option.is_none() {
            continue;
        }
        let entities = entities_option.unwrap();

        // For each entity
        for ((x, y, z), &entity) in entities {
            // Get sprite component
            if let Ok(mut sprite) = tiles.get_mut(entity) {
                // If selected position matches (highlight)
                if *x == player_tile_pos.x as usize && *y == player_tile_pos.y as usize {
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
}
