use bevy::prelude::*;

#[derive(Resource)]
pub struct ScalingFactor {
    pub factor: f32,
    pub pixel_factor: i32,
}
