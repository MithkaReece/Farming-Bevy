use bevy::prelude::*;

use crate::{
    components::{item_component::ItemType, Inventory, Plant, Player,
        Tile, Tilemap,
    },
    config::layer_enum::Layer,
    // systems::{get_ground_tile, get_object_tile_mut},
    resources::ScalingFactor,
};

pub fn plant_seed(
    input: Res<Input<KeyCode>>,
    mut inventory_query: Query<&mut Inventory>,
    player: Query<&Transform, With<Player>>,
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
        println!("Not selected item");
        return;
    }
    let selected_item_type = selected_item_type_option.unwrap();

    // Check item is seed
    if !matches!(selected_item_type, ItemType::Seed(_)) {
        println!("Item is not seed");
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

    // Check ground is hoed
    match tilemap.get_tile_with_layer(&chunk_pos, Layer::Ground, &tile_pos) {
        Some(ground_tile) => {
            if ground_tile != &Tile::Hoed {
                return;
            }
        }
        None => {
            println!("Not on ground tile (plant_seed_systems");
            return;
        }
    }

    // Check object tile is empty
    match tilemap.get_tile_with_layer(&chunk_pos, Layer::Object, &tile_pos) {
        Some(object_tile) => {
            if object_tile != &Tile::None {
                return;
            }
        }
        None => {
            println!("Not on object tile (plant_seed_systems");
            return;
        }
    }

    // Set object tile to selected seed
    let ItemType::Seed(seed) = selected_item_type;
    let new_tile = Tile::Seed(
        seed,
        Plant {
            stage: 0,
            max_stage: 4,
            growth_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
    );
    // println!("Planted{:?}", &new_tile);
    match tilemap.set_tile_with_layer(&chunk_pos, Layer::Object, &tile_pos, new_tile) {
        Ok(_) => println!("Hoe ground"),
        Err(e) => println!("{e}"),
    }
    inventory.remove(selected_item_type);
}
