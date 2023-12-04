use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub previous_looking_location: Vec2,
    pub looking_location: Vec2,
    pub current_state: PlayerState,
}

pub enum PlayerState {
    Idle,
    Moving,
    Interacting,
}
