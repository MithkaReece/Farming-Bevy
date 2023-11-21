use bevy::prelude::*;

use crate::{
    components::{item_component::ItemType, Inventory, PlantData, Player, TileType, Tilemap, GroundType, Tile},
    config::layer_enum::Layer,
    resources::ScalingFactor,
};

pub fn plant_seed(
    input: Res<Input<KeyCode>>,
    mut inventory_query: Query<&mut Inventory>,
    player: Query<&Player>,
    scaling_factor: Res<ScalingFactor>,
    mut tilemap: Query<&mut Tilemap>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    // Get selected item from inventory (if one exists)
    let mut inventory = inventory_query.single_mut();
    let selected_item_type_option = inventory.get_selected_item();
    if selected_item_type_option.is_none() {
        println!("No selected item");
        return;
    }
    let selected_item = selected_item_type_option.unwrap();

    // Check item is seed
    if !matches!(selected_item.item_type, ItemType::Seed(_)) {
        println!("Item is not seed");
        return;
    }

    let mut tilemap = tilemap.single_mut();

    let (chunk_pos, tile_pos) = tilemap.from_pos_no_layer(
        &player.single().looking_location,
        scaling_factor.get_full_factor(),
    );

    // Check ground is hoed
    match tilemap.get_tile_with_layer(&chunk_pos, Layer::Ground, &tile_pos) {
        Some(ground_tile) => {
            if ground_tile.tile_type != TileType::Ground(GroundType::Hoed) {
                return;
            }
        }
        None => {
            println!("Not on ground tile (plant_seed_systems");
            return;
        }
    }

    // Check object tile is empty
    if tilemap.get_tile_with_layer(&chunk_pos, Layer::Object, &tile_pos) != None {
        println!("Not on object tile (plant_seed_systems");
        return;
    }

    // Set object tile to selected seed
    let new_tile = match selected_item.item_type {
        ItemType::Seed(plant_type) => Tile {
            tile_type: TileType::Plant(
                plant_type,
                PlantData {
                    stage: 0,
                    max_stage: 4,
                    growth_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                    worth: 0.0,
                },
            ),
            has_collision: false
        },
        _ => return
    };
    // println!("Planted{:?}", &new_tile);
    match tilemap.set_tile_with_layer(&chunk_pos, Layer::Object, &tile_pos, new_tile) {
        Ok(_) => println!("Hoe ground"),
        Err(e) => println!("{e}"),
    }
    inventory.remove_selected();
}
