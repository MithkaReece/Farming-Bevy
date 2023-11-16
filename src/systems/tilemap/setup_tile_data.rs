use bevy::prelude::*;

use crate::resources::tile_info_resource::TilesData;

use std::fs;

pub fn setup_tile_data(mut tiles_data: ResMut<TilesData>,
    asset_server: Res<AssetServer>,
){
    let json_str = fs::read_to_string("assets/tiles_config.json");
    match json_str {
        Ok(data) => {
            let res: TilesData = serde_json::from_str(&data).unwrap();
            tiles_data.tiles = res.tiles;
        }
        Err(e) => {
            println!("Couldn't find tiles_config.json");
        }
    }
}