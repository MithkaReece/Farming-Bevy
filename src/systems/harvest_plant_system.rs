use std::time::Duration;

use bevy::prelude::*;

use crate::{
    components::{item_component::SeedType, Plant, Player, Tile, TileType},
    resources::{money_resource::Money, tilemap_resource::ObjectTilemap, ScalingFactor},
    systems::get_object_tile_mut,
    //systems::get_object_tile_id,
};

pub fn harvest_plant(
    input: Res<Input<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    mut object_tilemap: ResMut<ObjectTilemap>,
    scaling_factor: ResMut<ScalingFactor>,
    mut money: ResMut<Money>,
) {
    if !input.pressed(KeyCode::Space) {
        return;
    }
    // Calculate point we want to interact on
    let full_scaling_factor = scaling_factor.get_full_factor();
    let player_transform = player.single();
    let player_position = Vec2::new(
        player_transform.translation.x + scaling_factor.get_full_factor() / 2.0,
        player_transform.translation.y + full_scaling_factor / 2.0,
    );

    // Get object tile
    let mut object_tile_option = get_object_tile_mut(
        player_position,
        &mut object_tilemap,
        scaling_factor.get_full_factor(),
    );
    if object_tile_option.is_none() {
        println!("Not on object tile");
        return;
    }
    let mut object_tile = object_tile_option.unwrap();

    // Pattern match plant object and collect + sell
    match object_tile.tile_type {
        TileType::Seed(seed_type, ref mut plant) => {
            if plant.stage != plant.max_stage {
                return;
            }
            // Self produce
            plant.time_since_stage = Duration::from_secs(0);
            plant.stage = 1;
            // Map seed_type to money
            match seed_type {
                SeedType::Pumpkin => {
                    money.0 += 5.0;
                    println!("Money: {:?}", money.0);
                }
            }
        }
        _ => {
            return;
        }
    }
}
