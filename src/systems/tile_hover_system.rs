use bevy::prelude::*;

use crate::{
    components::{Player, Tile},
    resources::{tilemap_resource::GroundTilemap, ScalingFactor},
    systems::get_ground_tile_id,
};

pub fn tile_hover(
    player: Query<&Transform, With<Player>>,
    mut tiles: Query<(&mut TextureAtlasSprite, &Tile), Without<Player>>,
    tilemap: ResMut<GroundTilemap>,
    scaling_factor: ResMut<ScalingFactor>,
) {
    let full_scaling_factor = scaling_factor.get_full_factor();
    let player_transform = player.single();
    let player_position = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y - full_scaling_factor / 2.0,
    );

    let ground_tile_id_option = get_ground_tile_id(player_position, &tilemap, full_scaling_factor);
    if ground_tile_id_option.is_none() {
        return;
    }
    let ground_tile_id = ground_tile_id_option.unwrap();

    for (mut sprite, tile) in &mut tiles {
        if ground_tile_id == tile.unique_id {
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
