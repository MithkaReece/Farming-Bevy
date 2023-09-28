use bevy::prelude::*;

use super::CollisionType;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemType {
    Seed(SeedType),
    Collision(CollisionType),
}

impl ItemType {
    pub fn get_name(&self) -> String {
        match self {
            ItemType::Seed(seed_type) => seed_type.get_name(),
            ItemType::Collision(collision_type) => collision_type.get_name(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SeedType {
    Pumpkin,
    Carrot,
    Potato,
    Tomato,
}

impl SeedType {
    pub fn get_name(&self) -> String {
        match self {
            SeedType::Pumpkin => "Pumpkin".to_string(),
            SeedType::Carrot => "Carrot".to_string(),
            SeedType::Potato => "Potato".to_string(),
            SeedType::Tomato => "Tomato".to_string(),
        }
    }
}

#[derive(Component)]
pub struct Seed {
    pub seed_type: SeedType,
}
