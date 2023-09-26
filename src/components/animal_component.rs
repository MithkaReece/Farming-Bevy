use bevy::prelude::*;

#[derive(Component)]
pub struct Animal {
    pub animal_type: AnimalType,
    pub thirst: f32,
    pub hunger: f32,
    pub movement_speed: f32,
}

pub enum AnimalType {
    Sheep,
}
