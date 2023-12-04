use bevy::{prelude::*, sprite::collide_aabb::Collision};

/*
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
impl Eq for CollisionType {} */

#[derive(Component, Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub has_collision: bool,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.tile_type == other.tile_type
    }
}

// Define an enum to represent tile types
#[derive(Component, Debug, Clone)]
pub enum TileType {
    Ground(GroundType),
    Plant(PlantType, PlantData),
}

impl TileType {
    pub fn get_group_name(&self) -> String {
        match self {
            TileType::Ground(_) => "Ground".to_string(),
            TileType::Plant(_, _) => "Plant".to_string(),
        }
    }

    pub fn get_type_name(&self) -> String {
        match self {
            TileType::Ground(ground_type) => ground_type.get_name(),
            TileType::Plant(plant_type, _) => plant_type.get_name(),
        }
    }

    pub fn apply_index(&self, index: usize) -> usize {
        match &self {
            TileType::Plant(_, plant_data) => index + plant_data.stage,
            _ => index,
        }
    }
}

impl PartialEq for TileType {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            TileType::Ground(ground_type) => match other {
                TileType::Ground(ground_type_other) => ground_type == ground_type_other,
                _ => false,
            },
            TileType::Plant(plant_type, _) => match other {
                TileType::Plant(plant_type_other, _) => plant_type == plant_type_other,
                _ => false,
            },
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum GroundType {
    Grass,
    Hoed,
    Water,
}

impl GroundType {
    pub fn get_name(&self) -> String {
        match self {
            GroundType::Grass => "Grass".to_string(),
            GroundType::Hoed => "Hoed".to_string(),
            GroundType::Water => "Water".to_string(),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlantType {
    Pumpkin,
    Carrot,
    Potato,
    Tomato,
}

impl PlantType {
    pub fn get_name(&self) -> String {
        match self {
            PlantType::Pumpkin => "Pumpkin".to_string(),
            PlantType::Carrot => "Carrot".to_string(),
            PlantType::Potato => "Potato".to_string(),
            PlantType::Tomato => "Tomato".to_string(),
        }
    }
}

#[derive(Debug, Component, Clone, PartialEq)]
pub struct PlantData {
    pub stage: usize,
    pub max_stage: usize,
    pub growth_timer: Timer,
    pub worth: f64,
}
