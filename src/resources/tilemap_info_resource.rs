use bevy::prelude::*;

#[derive(Resource)]
pub struct TilemapInfo {
    pub chunk_size: usize, //No. tiles of a chunk width/height
    pub dimensions: UVec3,
}
