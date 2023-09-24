use bevy::prelude::*;

#[derive(Resource)]
pub struct TilemapInfo {
    pub chunk_size: usize,
    pub dimensions: UVec3,
}
