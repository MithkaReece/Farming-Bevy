use bevy::prelude::*;

use super::item_component::SeedType;

#[derive(Component)]
pub struct Tile {
    pub unique_id: usize,
    pub tile_type: TileType,
    pub visible: bool,
}

// Define an enum to represent tile types
#[derive(Debug, PartialEq, Eq)]
pub enum TileType {
    None,
    Grass,
    Hoed,
    Seed(SeedType),
}

impl Tile {
    pub fn get_index(&self) -> usize {
        match self.tile_type {
            TileType::Grass => 161,
            TileType::Hoed => 129,

            TileType::Seed(seed_type) => match seed_type {
                SeedType::Pumpkin => 1,
            },
            TileType::None => 0,
        }
    }

    pub fn get_spritesheet(&self) -> String {
        match self.tile_type {
            TileType::Grass => "farm_tilemap.png".to_string(),
            TileType::Hoed => "farm_tilemap.png".to_string(),
            TileType::Seed(_) => "plant.png".to_string(),
            TileType::None => "".to_string(),
        }
    }

    pub fn set_type(&mut self, tile_type: TileType) {
        if tile_type == TileType::None {
            self.visible = false;
        } else {
            self.visible = true;
        }
        self.tile_type = tile_type;
    }
}
