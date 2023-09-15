use bevy::prelude::*;

#[derive(Component)]
pub struct Plant {
    pub stage: usize,
    pub max_stage: usize,
    pub growth_counter: f32,
    pub time_between_stages: f32,
}