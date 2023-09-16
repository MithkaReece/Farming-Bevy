use bevy::{prelude::*, utils::HashMap};

use crate::components::tile_component::Tile;

pub const CHUNK_SIZE: usize = 16;

#[derive(Component, Clone)]
pub struct Chunk {
    //pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
    pub tiles: HashMap<(usize, usize), Tile>,
    pub position: Vec2,
    pub is_loaded: bool,
}

// Maps chunks to hashmap of tile entities
// Hash map used Vec2 position + Layer for key
#[derive(Resource)]
pub struct EntityChunkMapping {
    pub mapping: HashMap<(usize, usize), HashMap<(usize, usize, usize), Entity>>,
}
