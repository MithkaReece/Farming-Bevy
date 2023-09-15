use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemType {
    Seed(SeedType),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SeedType {
    Pumpkin,
}

#[derive(Component)]
pub struct Seed {
    pub seed_type: SeedType,
}
