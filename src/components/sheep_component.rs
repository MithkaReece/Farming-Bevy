use bevy::prelude::*;

#[derive(Component)]
pub struct Sheep {
    pub hunger: f32,
    pub movement_speed: f32,
}
