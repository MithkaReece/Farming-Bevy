use bevy::prelude::*;

use crate::{
    components::{
        item_component::ItemType, GroundType, Inventory, PlantData, Player, Tile, TileType, Tilemap,
    },
    config::layer_enum::TilemapLayer,
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
    let player = player.single();
    let real_pos = Vec2::new(player.looking_location.x, player.looking_location.y);

    // Check ground is hoed
    match tilemap.get_tile_from_real(&real_pos, TilemapLayer::Ground, scaling_factor.full()) {
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
    if tilemap.get_tile_from_real(&real_pos, TilemapLayer::Object, scaling_factor.full()) != None {
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
            has_collision: false,
        },
        _ => return,
    };
    // println!("Planted{:?}", &new_tile);
    match tilemap.set_tile_from_real(
        &real_pos,
        TilemapLayer::Object,
        scaling_factor.full(),
        new_tile,
    ) {
        Ok(_) => println!("Hoe ground"),
        Err(e) => println!("{e}"),
    }
    inventory.remove_selected();
}
