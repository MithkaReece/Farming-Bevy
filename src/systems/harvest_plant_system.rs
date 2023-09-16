use bevy::prelude::*;

use crate::{
    components::{Plant, Player, Tile},
    resources::{money_resource::Money, tilemap_resource::ObjectTilemap, ScalingFactor},
    //systems::get_object_tile_id,
};

pub fn harvest_plant(
    input: Res<Input<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    mut tiles: Query<(&mut Plant, &Tile)>,
    object_tilemap: ResMut<ObjectTilemap>,
    scaling_factor: ResMut<ScalingFactor>,
    mut money: ResMut<Money>,
) {
    // if !input.pressed(KeyCode::Space) {
    //     return;
    // }
    // // Calculate point we want to interact on
    // let full_scaling_factor = scaling_factor.factor * scaling_factor.pixel_factor as f32;
    // let player_transform = player.single();
    // let player_position = Vec2::new(
    //     player_transform.translation.x,
    //     player_transform.translation.y - full_scaling_factor / 2.0,
    // );

    // // Check we are on a object tile (retrieve tile_id from hashmap)
    // let object_tile_id_option =
    //     get_object_tile_id(player_position, &object_tilemap, full_scaling_factor);
    // if object_tile_id_option.is_none() {
    //     println!("Not on object tile");
    //     return;
    // }
    // let object_tile_id = object_tile_id_option.unwrap();

    // // Retrieve object tile
    // let mut plant_option: Option<Mut<'_, Plant>> = None;
    // //let mut object_tile_option: Option<&Tile> = None;
    // for (plant, tile) in &mut tiles {
    //     if object_tile_id == tile.unique_id {
    //         plant_option = Some(plant);
    //         //object_tile_option = Some(tile);
    //         break;
    //     }
    // }
    // if plant_option.is_none()
    // /* || object_tile_option.is_none()*/
    // {
    //     println!("Object or entity not found");
    //     return;
    // }
    // let mut plant = plant_option.unwrap();
    // //let object_tile = object_tile_option.unwrap();

    // // Check you can harvest the plant
    // if plant.stage != plant.max_stage {
    //     return;
    // }
    // // Self produce
    // plant.stage = 1;
    // money.0 += 5.0;
    // println!("Money: {:?}", money.0);
}
