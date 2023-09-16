use bevy::prelude::*;

use crate::{
    components::{chunk_component::CHUNK_SIZE, Player, Tile, TileType},
    resources::{tilemap_resource::GroundTilemap, tilemap_resource::ObjectTilemap, ScalingFactor},
};

pub fn hoe_ground(
    player: Query<&Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    object_tilemap: ResMut<ObjectTilemap>,
    mut ground_tilemap: ResMut<GroundTilemap>,
    scaling_factor: ResMut<ScalingFactor>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let full_scaling_factor = scaling_factor.get_full_factor();
    let player_transform = player.single();
    let player_position = Vec2::new(
        player_transform.translation.x+ scaling_factor.get_full_factor() / 2.0,
        player_transform.translation.y + full_scaling_factor / 2.0,
    );

    // Get object tile
    let object_tile_option = get_object_tile(
        player_position,
        &object_tilemap,
        scaling_factor.get_full_factor(),
    );
    if object_tile_option.is_none() {
        println!("Not on object tile");
        return;
    }
    let object_tile = object_tile_option.unwrap();
    // Check object is empty
    if object_tile.tile_type != TileType::None {
        return;
    }

    // Get ground tile
    let mut ground_tile_option = get_ground_tile_mut(
        player_position,
        &mut ground_tilemap,
        scaling_factor.get_full_factor(),
    );
    if let Some(ground_tile) = ground_tile_option.as_mut() {
        if ground_tile.tile_type == TileType::Grass {
            ground_tile.set_type(TileType::Hoed);
            println!("Hoe ground");
        }
    } else {
        println!("Not on ground tile");
        return;
    }
}

pub fn get_ground_tile(pos: Vec2, tilemap: &GroundTilemap, scaling_factor: f32) -> Option<&Tile> {
    // Find chunk
    let chunk_pos = pos_to_chunk_pos(pos, scaling_factor);
    // Extract tile from chunk
    let chunk_option = tilemap
        .tiles
        .get(&(chunk_pos.x as usize, chunk_pos.y as usize));
    if chunk_option.is_none() {
        return None;
    }
    let chunk = chunk_option.unwrap();
    let tile_pos = pos_to_tile_pos(pos, scaling_factor);
    chunk.tiles.get(&(tile_pos.x as usize, tile_pos.y as usize))
}

pub fn get_ground_tile_mut(
    pos: Vec2,
    tilemap: &mut GroundTilemap,
    scaling_factor: f32,
) -> Option<&mut Tile> {
    // Find chunk
    let chunk_pos = pos_to_chunk_pos(pos, scaling_factor);
    // Extract tile from chunk
    let chunk_option = tilemap
        .tiles
        .get_mut(&(chunk_pos.x as usize, chunk_pos.y as usize));
    if chunk_option.is_none() {
        return None;
    }
    let chunk = chunk_option.unwrap();
    let tile_pos = pos_to_tile_pos(pos, scaling_factor);
    chunk
        .tiles
        .get_mut(&(tile_pos.x as usize, tile_pos.y as usize))
}

pub fn get_object_tile(pos: Vec2, tilemap: &ObjectTilemap, scaling_factor: f32) -> Option<&Tile> {
    // Find chunk
    let chunk_pos = pos_to_chunk_pos(pos, scaling_factor);
    // println!("Chunk Pos: ({:?}, {:?})", chunk_pos.x, chunk_pos.y);
    // Extract tile from chunk
    let chunk_option = tilemap
        .tiles
        .get(&(chunk_pos.x as usize, chunk_pos.y as usize));
    if chunk_option.is_none() {
        return None;
    }
    let chunk = chunk_option.unwrap();
    let tile_pos = pos_to_tile_pos(pos, scaling_factor);
    println!(
        "Tile-pos: ({:?}, {:?})",
        tile_pos.x as usize, tile_pos.y as usize,
    );
    chunk.tiles.get(&(tile_pos.x as usize, tile_pos.y as usize))
}

pub fn get_object_tile_mut(
    pos: Vec2,
    tilemap: &mut ObjectTilemap,
    scaling_factor: f32,
) -> Option<&mut Tile> {
    // Find chunk
    let chunk_pos = pos_to_chunk_pos(pos, scaling_factor);
    // Extract tile from chunk
    let chunk_option = tilemap
        .tiles
        .get_mut(&(chunk_pos.x as usize, chunk_pos.y as usize));
    if chunk_option.is_none() {
        return None;
    }
    let chunk = chunk_option.unwrap();
    let tile_pos = pos_to_tile_pos(pos, scaling_factor);
    chunk
        .tiles
        .get_mut(&(tile_pos.x as usize, tile_pos.y as usize))
}

pub fn pos_to_chunk_pos(pos: Vec2, scaling_factor: f32) -> Vec2 {
    Vec2::new(
        (CHUNK_SIZE as f32 * scaling_factor)
            * (pos.x / (CHUNK_SIZE as f32 * scaling_factor)).floor(),
        (CHUNK_SIZE as f32 * scaling_factor)
            * (pos.y / (CHUNK_SIZE as f32 * scaling_factor)).floor(),
    )
}

pub fn pos_to_tile_pos(pos: Vec2, scaling_factor: f32) -> Vec2 {
    Vec2::new(
        (pos.x / scaling_factor).floor() * scaling_factor,
        (pos.y / scaling_factor).floor() * scaling_factor,
    )
}
