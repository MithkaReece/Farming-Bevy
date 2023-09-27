use bevy::prelude::*;

use crate::{
    components::{Player, Tile, Tilemap},
    config::layer_enum::Layer,
    resources::ScalingFactor,
};

pub fn hoe_ground(
    player: Query<&Player>,
    input: Res<Input<KeyCode>>,
    scaling_factor: ResMut<ScalingFactor>,
    mut tilemap: Query<&mut Tilemap>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let mut tilemap = tilemap.single_mut();

    let (chunk_pos, tile_pos) = tilemap.from_pos_no_layer(
        &player.single().looking_location,
        scaling_factor.get_full_factor(),
    );
    // Check no object tile in the way of hoeing
    match tilemap.get_tile_with_layer(&chunk_pos, Layer::Object, &tile_pos) {
        Some(object_tile) => {
            if object_tile != &Tile::None {
                return;
            }
        }
        None => {
            println!("could not find object tile to hoe (hoe_ground_system)");
            return;
        }
    }

    // Check ground is grass
    match tilemap.get_tile_with_layer(&chunk_pos, Layer::Ground, &tile_pos) {
        Some(object_tile) => {
            if object_tile != &Tile::Grass {
                return;
            }
        }
        None => {
            println!("could not find grass tile to hoe (hoe_ground_system)");
            return;
        }
    }

    match tilemap.set_tile_with_layer(&chunk_pos, Layer::Ground, &tile_pos, Tile::Hoed) {
        Ok(_) => println!("Hoe ground"),
        Err(e) => println!("{e}"),
    }
}
