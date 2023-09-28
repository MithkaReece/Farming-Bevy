use bevy::{prelude::*, sprite::collide_aabb::Collision};

use super::item_component::SeedType;

#[derive(Debug, Component, Clone)]
pub struct PlantData {
    pub stage: usize,
    pub max_stage: usize,
    pub growth_timer: Timer,
}
#[derive(Debug, Component, Clone, Copy)]
pub struct FenceData {}

#[derive(Debug, Component, Clone, Copy)]
pub enum CollisionType {
    Fence(FenceData),
}

impl CollisionType {
    pub fn get_name(&self)->String{
        match self {
            CollisionType::Fence(_) => {"Fence".to_string()}
        }
    }
}

impl PartialEq for CollisionType {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            CollisionType::Fence(_) => match other {
                CollisionType::Fence(_) => true,
                _ => false,
            },
        }
    }
}
impl Eq for CollisionType {} 

// Define an enum to represent tile types
#[derive(Component, Debug, Clone)]
pub enum Tile {
    None,
    Grass,
    Hoed,
    Seed(SeedType, PlantData),
    Collision(CollisionType),
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            // Compare seed_types
            Tile::Seed(seed_type, _) => match other {
                Tile::Seed(seed_type_other, _) => seed_type == seed_type_other,
                _ => false,
            },
            // Compare collision type without meta data
            Tile::Collision(_) => match other {
                Tile::Collision(_) => true,
                _ => false,
            },
            // Normal eq compare
            Tile::None => match other {
                Tile::None => true,
                _ => false,
            },
            Tile::Grass => match other {
                Tile::Grass => true,
                _ => false,
            },
            Tile::Hoed => match other {
                Tile::Hoed => true,
                _ => false,
            },
        }
    }
}

impl Tile {
    pub fn get_index(&self) -> usize {
        match &self {
            Tile::None => 0,
            Tile::Grass => 161,
            Tile::Hoed => 129,

            Tile::Seed(seed_type, plant) => match seed_type {
                SeedType::Pumpkin => 0 + plant.stage,
                SeedType::Carrot => 5 + plant.stage,
                SeedType::Potato => 10 + plant.stage,
                SeedType::Tomato => 15 + plant.stage,
            },
            //TODO::Map these
            Tile::Collision(collision_type) => match collision_type {
                CollisionType::Fence(_) => 166,
            },
        }
    }

    // Note in sync_tile_visual I map tile to sprite atlas handlerr
    // pub fn get_spritesheet(&self) -> String {
    //     match &self {
    //         Tile::None => "".to_string(),
    //         Tile::Grass => "farm_tilemap.png".to_string(),
    //         Tile::Hoed => "farm_tilemap.png".to_string(),
    //         Tile::Seed(_, _) => "plant.png".to_string(),
    //         Tile::Collision(_) => "".to_string(),
    //     }
    // }
}
