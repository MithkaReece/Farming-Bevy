use bevy::prelude::*;

use std::collections::HashMap;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct TileData {
    #[serde(rename = "type")]
    pub tile_type: String,
    pub spritesheet: String,
    pub sprite_index: usize,
}

#[derive(Resource, Debug, Deserialize)]
pub struct TilesData {
    pub tiles: HashMap<String, Vec<TileData>>,
}


impl TilesData {
    pub fn get_tile(&self, group:&str, tile_type:&str) -> Option<&TileData> {
        let tiles = match self.tiles.get(group) {
            Some(tiles) => { tiles }
            None => { return None; }
        };
    
        tiles.iter().find(|&tile| tile.tile_type == tile_type)
    }
}