use bevy::prelude::*;

use super::CollisionType;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemType {
    Seed(SeedType),
    Collision(CollisionType),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SeedType {
    Pumpkin,
    Carrot,
    Potato,
    Tomato,
}

#[derive(Component)]
pub struct Seed {
    pub seed_type: SeedType,
}
