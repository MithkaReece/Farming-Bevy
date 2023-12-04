use bevy::prelude::*;

use crate::config::layer_enum::TilemapLayer;

use super::Tilemap;

#[derive(Component, Clone)]
pub struct Chunk {
    pub chunk_bottom_left_pos: UVec2,
    pub chunk_size: usize,
    pub is_loaded: bool,
}

impl Chunk {
    pub fn new(chunk_bottom_left_pos: UVec2, chunk_size: usize) -> Self {
        Chunk {
            chunk_bottom_left_pos,
            chunk_size,
            is_loaded: false,
        }
    }
}
