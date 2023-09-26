use bevy::prelude::*;

use crate::{
    components::{item_component::SeedType, Player, Tile, Tilemap},
    resources::{money_resource::Money, ScalingFactor}, config::layer_enum::Layer,
};

pub fn harvest_plant(
    input: Res<Input<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    scaling_factor: Res<ScalingFactor>,
    mut money: ResMut<Money>,
    mut tilemap: Query<&mut Tilemap>,
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

    let mut tilemap = tilemap.single_mut();

    let (chunk_pos, tile_pos) =
        tilemap.from_pos_no_layer(&player_position, scaling_factor.get_full_factor());

    let object_tile = match tilemap.get_tile_mut(&UVec3::new(chunk_pos.x,chunk_pos.y, Layer::Object as u32), &tile_pos) {
        Some(tile) => tile,
        None => {
            println!("Harvest can't find tile");
            return
        }
    };

    // Pattern match plant object and collect + sell
    match object_tile {
        Tile::Seed(seed_type, ref mut plant) => {
            if plant.stage != plant.max_stage {
                return;
            }
            // Self produce
            plant.growth_timer.reset();
            plant.stage = 1;
            // Map seed_type to money
            match seed_type {
                SeedType::Pumpkin => {
                    money.0 += 5.0;
                    println!("Money: {:?}", money.0);
                }
                SeedType::Carrot => {
                    money.0 += 5.0;
                    println!("Money: {:?}", money.0);
                }
                SeedType::Potato => {
                    money.0 += 5.0;
                    println!("Money: {:?}", money.0);
                }
                SeedType::Tomato => {
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
