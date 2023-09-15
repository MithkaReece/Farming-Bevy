use bevy::prelude::*;

use crate::{
    components::{Player, Tile, TileType},
    resources::{tilemap_resource::GroundTilemap, tilemap_resource::ObjectTilemap, ScalingFactor},
};

pub fn hoe_ground(
    player: Query<&Transform, With<Player>>,
    mut tiles: Query<&mut Tile>,
    input: Res<Input<KeyCode>>,
    object_tilemap: ResMut<ObjectTilemap>,
    ground_tilemap: ResMut<GroundTilemap>,
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

    // Check object is empty
    if let Some(tile_id) = get_object_tile_id(player_position, &object_tilemap, full_scaling_factor)
    {
        for (tile) in &tiles {
            if tile_id == tile.unique_id {
                if tile.visible {
                    return;
                }
                break;
            }
        }
    }
    // Hoe grass tile
    if let Some(tile_id) = get_ground_tile_id(player_position, &ground_tilemap, full_scaling_factor)
    {
        for (mut tile) in &mut tiles {
            if tile_id == tile.unique_id {
                if tile.tile_type == TileType::Grass {
                    tile.set_type(TileType::Hoed);
                    return;
                }
                break;
            }
        }
    }
}

pub fn get_ground_tile_id(
    pos: Vec2,
    tilemap: &GroundTilemap,
    full_scaling_factor: f32,
) -> Option<usize> {
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

pub fn get_object_tile_id(
    pos: Vec2,
    tilemap: &ObjectTilemap,
    full_scaling_factor: f32,
) -> Option<usize> {
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
