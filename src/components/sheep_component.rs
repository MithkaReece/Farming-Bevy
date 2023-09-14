use bevy::prelude::*;

#[derive(Component)]
pub struct Sheep {
    pub lifetime: Timer,
    pub movement_speed: f32,
}
