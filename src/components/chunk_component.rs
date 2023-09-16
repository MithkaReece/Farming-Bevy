use bevy::{prelude::*, utils::HashMap};

use crate::components::tile_component::Tile;

pub const CHUNK_SIZE: usize = 16;

#[derive(Component, Clone, Copy)]
pub struct Chunk {
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
    pub position: Vec2,
    pub is_loaded: bool,
}

#[derive(Resource)]
pub struct EntityChunkMapping {
    pub mapping: HashMap<(usize, usize), Vec<Entity>>,
}
