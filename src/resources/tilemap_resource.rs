use bevy::{prelude::*, utils::HashMap};

use crate::components::chunk_component::Chunk;

#[derive(Resource)]
pub struct GroundTilemap {
    pub tiles: HashMap<(usize, usize), Chunk>,
    pub num_chunks_width: usize,
    pub num_chunks_height: usize,
}

#[derive(Resource)]
pub struct ObjectTilemap {
    pub tiles: HashMap<(usize, usize), Chunk>,
    pub num_chunks_width: usize,
    pub num_chunks_height: usize,
}
