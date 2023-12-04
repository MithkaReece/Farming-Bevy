use bevy::prelude::*;

use crate::{
    components::{GroundType, Player, Tile, TileType, Tilemap},
    config::layer_enum::TilemapLayer,
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
    let player = player.single();
    let real_pos = Vec2::new(player.looking_location.x, player.looking_location.y);

    // Check no object tile in the way of hoeing
    if tilemap.get_tile_from_real(&real_pos, TilemapLayer::Object, scaling_factor.full()) != None {
        println!(
            "could not find object tile to hoe (hoe_ground_system) at ({:?} {:?})",
            player.looking_location.x, player.looking_location.y
        );
        return;
    }

    // Check ground is grass
    match tilemap.get_tile_from_real(&real_pos, TilemapLayer::Ground, scaling_factor.full()) {
        Some(object_tile) => {
            if object_tile.tile_type != TileType::Ground(GroundType::Grass) {
                return;
            }
        }
        None => {
            println!("could not find grass tile to hoe (hoe_ground_system)");
            return;
        }
    }

    let new_tile = Tile {
        tile_type: TileType::Ground(GroundType::Hoed),
        has_collision: false,
    };

    match tilemap.set_tile_from_real(
        &real_pos,
        TilemapLayer::Ground,
        scaling_factor.full(),
        new_tile,
    ) {
        Ok(_) => println!("Hoe ground"),
        Err(e) => println!("{e}"),
    }
}
