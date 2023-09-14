use bevy::prelude::*;

#[derive(Component)]
pub struct Sheep {
    pub lifetime: Timer,
}