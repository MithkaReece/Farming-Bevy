use bevy::prelude::*;

use crate::{
    components::{Player, Tile},
    resources::{tilemap_resource::Tilemap, ScalingFactor},
};

pub fn hoe_ground(
    player: Query<&Transform, With<Player>>,
    mut tiles: Query<(&mut TextureAtlasSprite, &Tile), Without<Player>>,
    input: Res<Input<KeyCode>>,
    mut tilemap: ResMut<Tilemap>,
    scaling_factor: ResMut<ScalingFactor>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    let full_scaling_factor = scaling_factor.factor * scaling_factor.pixel_factor as f32;
    let player_transform = player.single();
    let player_position = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y - full_scaling_factor / 2.0,
    );

    if let Some(tile_id) = get_tile_id(player_position, &tilemap, full_scaling_factor) {
        for (mut sprite, tile) in &mut tiles {
            if tile_id == tile.unique_id {
                sprite.index += 1;
                return;
            }
        }
    }
}

pub fn get_tile_id(pos: Vec2, tilemap: &Tilemap, full_scaling_factor: f32) -> Option<usize> {
    let target_position = (pos / full_scaling_factor).round() * full_scaling_factor;
    if let Some(tile_found) = tilemap
        .tiles
        .iter()
        .find(|tile| tile.position == target_position)
    {
        Some(tile_found.unique_id)
    } else {
        None
    }
}
