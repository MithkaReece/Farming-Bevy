use bevy::prelude::*;

#[derive(Resource)]
pub struct ScalingFactor {
    pub factor: f32,
    pub pixel_factor: i32,
}

impl ScalingFactor {
    pub fn get_full_factor(&self) -> f32 {
        self.factor * self.pixel_factor as f32
    }
}
