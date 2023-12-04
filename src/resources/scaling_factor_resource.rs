use bevy::prelude::*;

#[derive(Resource)]
pub struct ScalingFactor {
    pub factor: f32,
    pub tile_size: i32,
}

impl ScalingFactor {
    pub fn full(&self) -> f32 {
        self.factor * self.tile_size as f32
    }
}
