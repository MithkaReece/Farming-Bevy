use bevy::prelude::*;

use crate::{
    components::{Player, Tile},
    resources::{tilemap_resource::GroundTilemap, ScalingFactor},
    systems::get_ground_tile_id,
};

pub fn tile_hover(
    player: Query<&Transform, With<Player>>,
    mut tiles: Query<(&mut TextureAtlasSprite, &Tile), Without<Player>>,
    mut tilemap: ResMut<GroundTilemap>,
    scaling_factor: ResMut<ScalingFactor>,
) {
    let full_scaling_factor = scaling_factor.factor * scaling_factor.pixel_factor as f32;
    let player_transform = player.single();
    let player_position = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y - full_scaling_factor / 2.0,
    );

    if let Some(tile_id) = get_ground_tile_id(player_position, &tilemap, full_scaling_factor) {
        for (mut sprite, tile) in &mut tiles {
            if tile_id == tile.unique_id {
                sprite.color = Color::Rgba {
                    red: 0.9,
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
