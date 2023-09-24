use bevy::prelude::*;

use super::item_component::SeedType;


#[derive(Debug, PartialEq, Eq, Component, Clone)]
pub struct Plant {
    pub stage: usize,
    pub max_stage: usize,
    pub growth_timer: Timer,
}

// Define an enum to represent tile types
#[derive(Component, Debug, PartialEq, Eq, Clone)]
pub enum Tile {
    None,
    Grass,
    Hoed,
    Seed(SeedType, Plant),
}

impl Tile {
    pub fn get_index(&self) -> usize {
        match &self {
            Tile::Grass => 161,
            Tile::Hoed => 129,

            Tile::Seed(seed_type, plant) => match seed_type {
                SeedType::Pumpkin => 0 + plant.stage,
                SeedType::Carrot => 5 + plant.stage,
                SeedType::Potato => 10 + plant.stage,
                SeedType::Tomato => 15 + plant.stage,
            },
            Tile::None => 0,
        }
    }

    pub fn get_spritesheet(&self) -> String {
        match &self {
            Tile::Grass => "farm_tilemap.png".to_string(),
            Tile::Hoed => "farm_tilemap.png".to_string(),
            Tile::Seed(_, _) => "plant.png".to_string(),
            Tile::None => "".to_string(),
        }
    }
}
