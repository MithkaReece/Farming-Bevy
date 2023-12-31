use bevy::prelude::*;

use crate::{
    components::{PlantType, Player, TileType, Tilemap},
    config::layer_enum::TilemapLayer,
    resources::{money_resource::Money, ScalingFactor},
};

pub fn harvest_plant(
    input: Res<Input<KeyCode>>,
    player: Query<&Player>,
    scaling_factor: Res<ScalingFactor>,
    mut money: ResMut<Money>,
    mut tilemap: Query<&mut Tilemap>,
) {
    if !input.pressed(KeyCode::Space) {
        return;
    }

    let mut tilemap = tilemap.single_mut();
    let player = player.single();

    let object_tile = match tilemap.get_tile_mut_from_real(
        &Vec2::new(player.looking_location.x, player.looking_location.y),
        TilemapLayer::Object,
        scaling_factor.full(),
    ) {
        Some(tile) => tile,
        None => {
            //println!("Harvest can't find tile");
            return;
        }
    };

    // Pattern match plant object and collect + sell
    match object_tile.tile_type {
        TileType::Plant(plant_type, ref mut plant) => {
            if plant.stage != plant.max_stage {
                return;
            }
            // Self produce
            plant.growth_timer.reset();
            plant.stage = 1;
            // Map seed_type to money
            match plant_type {
                PlantType::Pumpkin => {
                    money.0 += 5.0;
                    println!("Money: {:?}", money.0);
                }
                PlantType::Carrot => {
                    money.0 += 5.0;
                    println!("Money: {:?}", money.0);
                }
                PlantType::Potato => {
                    money.0 += 5.0;
                    println!("Money: {:?}", money.0);
                }
                PlantType::Tomato => {
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
