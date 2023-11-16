use bevy::prelude::*;

use super::PlantType;

//use super::CollisionType;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemType {
    Seed(PlantType),
    //Collision(CollisionType),
}

impl ItemType {
    pub fn get_name(&self) -> String {
        match self {
            ItemType::Seed(plant_type) => plant_type.get_name(),
            //ItemType::Collision(collision_type) => collision_type.get_name(),
        }
    }
}