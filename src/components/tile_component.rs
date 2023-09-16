use std::time::{Duration, Instant};

use bevy::prelude::*;

use super::item_component::SeedType;

#[derive(Component, Clone)]
pub struct Tile {
    pub position: Vec3,
    pub tile_type: TileType,
    pub visible: bool,
    pub index_offset: usize,
}

#[derive(Debug, PartialEq, Eq, Component, Clone)]
pub struct Plant {
    pub stage: usize,
    pub max_stage: usize,
    pub growth_timer: Timer,
}

// Define an enum to represent tile types
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TileType {
    None,
    Grass,
    Hoed,
    Seed(SeedType, Plant),
}

impl Tile {
    pub fn get_index(&self) -> usize {
        match &self.tile_type {
            TileType::Grass => 161 + self.index_offset,
            TileType::Hoed => 129 + self.index_offset,

            TileType::Seed(seed_type, plant) => match seed_type {
                SeedType::Pumpkin => 0 + self.index_offset + plant.stage,
            },
            TileType::None => 0,
        }
    }

    pub fn get_spritesheet(&self) -> String {
        match self.tile_type {
            TileType::Grass => "farm_tilemap.png".to_string(),
            TileType::Hoed => "farm_tilemap.png".to_string(),
            TileType::Seed(_, _) => "plant.png".to_string(),
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
