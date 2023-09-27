use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub looking_location: Vec2,
}
