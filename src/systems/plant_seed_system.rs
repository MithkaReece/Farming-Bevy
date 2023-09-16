use std::time::{Duration};

use bevy::prelude::*;

use crate::{
    components::{
        chunk_component::EntityChunkMapping, item_component::ItemType, Inventory, Plant, Player,
        Tile, TileType,
    },
    resources::{
        tilemap_resource::{GroundTilemap, ObjectTilemap},
        ScalingFactor,
    },
    systems::{get_ground_tile, get_object_tile_mut},
};

pub fn plant_seed(
    input: Res<Input<KeyCode>>,
    mut inventory_query: Query<&mut Inventory>,
    player: Query<&Transform, With<Player>>,
    mut object_tilemap: ResMut<ObjectTilemap>,
    ground_tilemap: Res<GroundTilemap>,
    entity_chunk_map: Res<EntityChunkMapping>,
    scaling_factor: Res<ScalingFactor>,
    mut commands: Commands,
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

    // Retrieve ground tile
    let ground_tile_option = get_ground_tile(player_position, &ground_tilemap, full_scaling_factor);
    if ground_tile_option.is_none() {
        println!("Not on ground tile");
        return;
    }
    // Check if ground is hoed
    let ground_tile = ground_tile_option.unwrap();
    if ground_tile.tile_type != TileType::Hoed {
        println!("Ground not hoed");
        return;
    }

    // Get object tile
    let mut object_tile_option =
        get_object_tile_mut(player_position, &mut object_tilemap, full_scaling_factor);
    if object_tile_option.is_none() {
        println!("Not on object tile");
        return;
    }
    let mut object_tile = object_tile_option.unwrap();
    // Make sure object is empty
    if object_tile.tile_type != TileType::None {
        println!("Object tile not empty");
        return;
    }

    // Change tile type to seed from item type
    let ItemType::Seed(seed) = selected_item_type;
    object_tile.set_type(TileType::Seed(
        seed,
        Plant {
            stage: 0,
            max_stage: 4,
            time_since_stage: Duration::from_secs(0),
            time_between_stages: Duration::from_secs(1),
        },
    ));
    println!("{:?}", object_tile.tile_type);

    // Remove seed as it has been planted
    inventory.remove(selected_item_type);
}
