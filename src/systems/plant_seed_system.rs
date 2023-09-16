use bevy::prelude::*;

use crate::{
    components::{item_component::ItemType, Inventory, Plant, Player, Tile, TileType},
    resources::{
        tilemap_resource::{GroundTilemap, ObjectTilemap},
        ScalingFactor,
    },
    //systems::{get_ground_tile_id, get_object_tile_id},
};

pub fn plant_seed(
    input: Res<Input<KeyCode>>,
    mut inventory_query: Query<&mut Inventory>,
    player: Query<&Transform, With<Player>>,
    mut tiles: Query<(&mut Tile, Entity)>,
    object_tilemap: ResMut<ObjectTilemap>,
    ground_tilemap: ResMut<GroundTilemap>,
    scaling_factor: ResMut<ScalingFactor>,
    mut commands: Commands,
) {
    // if !input.just_pressed(KeyCode::Space) {
    //     return;
    // }

    // // Get selected item from inventory (if one exists)
    // let mut inventory = inventory_query.single_mut();
    // let selected_item_type_option = inventory.get_selected_item();
    // if selected_item_type_option.is_none() {
    //     println!("Not selected item");
    //     return;
    // }
    // let selected_item_type = selected_item_type_option.unwrap();

    // // Check item is seed
    // if !matches!(selected_item_type, ItemType::Seed(_)) {
    //     println!("Item is not seed");
    //     return;
    // }

    // // Calculate point we want to interact on
    // let full_scaling_factor = scaling_factor.factor * scaling_factor.pixel_factor as f32;
    // let player_transform = player.single();
    // let player_position = Vec2::new(
    //     player_transform.translation.x,
    //     player_transform.translation.y - full_scaling_factor / 2.0,
    // );

    // // Check we are on a ground tile (retrieve tile_id from hashmap)
    // let ground_tile_id_option =
    //     get_ground_tile_id(player_position, &ground_tilemap, full_scaling_factor);
    // if ground_tile_id_option.is_none() {
    //     println!("Not on ground tile");
    //     return;
    // }
    // let ground_tile_id = ground_tile_id_option.unwrap();

    // // Retrieve ground tile
    // let mut ground_tile_option: Option<&Tile> = None;
    // for (tile, _) in &tiles {
    //     if ground_tile_id == tile.unique_id {
    //         ground_tile_option = Some(tile);
    //         break;
    //     }
    // }
    // if ground_tile_option.is_none() {
    //     println!("Ground tile not found");
    //     return;
    // }
    // let ground_tile = ground_tile_option.unwrap();

    // // Check ground is hoed
    // if ground_tile.tile_type != TileType::Hoed {
    //     println!("Ground not hoed");
    //     return;
    // }

    // // Check we are on a object tile (retrieve tile_id from hashmap)
    // let object_tile_id_option =
    //     get_object_tile_id(player_position, &object_tilemap, full_scaling_factor);
    // if object_tile_id_option.is_none() {
    //     println!("Not on object tile");
    //     return;
    // }
    // let object_tile_id = object_tile_id_option.unwrap();

    // // Retrieve object tile
    // let mut object_tile_option: Option<Mut<'_, Tile>> = None;
    // let mut entity_option: Option<Entity> = None;
    // for (tile, entity) in &mut tiles {
    //     if object_tile_id == tile.unique_id {
    //         object_tile_option = Some(tile);
    //         entity_option = Some(entity);
    //         break;
    //     }
    // }
    // if object_tile_option.is_none() || entity_option.is_none() {
    //     println!("Object or entity not found");
    //     return;
    // }
    // let mut object_tile = object_tile_option.unwrap();
    // let mut entity = entity_option.unwrap();

    // // Check if object tile is empty
    // if object_tile.tile_type != TileType::None {
    //     println!("Object tile not empty");
    //     return;
    // }

    // // Change tile type to seed from item type
    // let ItemType::Seed(seed) = selected_item_type;
    // object_tile.set_type(TileType::Seed(seed));
    // println!("{:?}", object_tile.tile_type);

    // commands.entity(entity).insert(Plant {
    //     stage: 0,
    //     max_stage: 4,
    //     growth_counter: 0.0,
    //     time_between_stages: 1.0,
    // });

    // // Remove seed as it has been planted
    // inventory.remove(selected_item_type);
}
