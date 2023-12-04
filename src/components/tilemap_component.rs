use bevy::prelude::*;

use crate::{
    components::{chunk_component::Chunk, tile_component::Tile},
    config::layer_enum::TilemapLayer,
};

#[derive(Component)]
pub struct Tilemap {
    pub dimensions: UVec3,
    pub chunk_size: usize, //No. tiles of a chunk width/height
    pub chunks: Vec<Vec<Vec<Chunk>>>,
}

/**
 * Tilemap holds chunks
 * Chunk hold tiles
 * a
 *
 *
 * Position 2D.
 * Tilemap.get_tile(position)
 * Reduce position down to chunk position
 * (between 0,0 and 1,1 is chunk 0,0)
 * Retrieve chunk
 * Take away chunk so reduced position is between (0,0) and (1,1)
 * Scale position up by chunk_size, round down to integer for tile position
 * retrieve tile from chunk
 * return tile
 *
 *
 * Get 2D position of tile
 * Tilemap.get_tile_position(chunk_position, tile_position)
 * do reverse of get_tile process to map to real coordinate
 * don't forget scaling factor
 *
 */

impl Tilemap {
    pub fn new(dimensions: UVec3, chunk_size: usize) -> Self {
        let chunks =
            vec![
                vec![vec![Chunk::new(chunk_size); dimensions.z as usize]; dimensions.y as usize];
                dimensions.x as usize
            ];

        Tilemap {
            dimensions,
            chunk_size,
            chunks,
        }
    }

    pub fn get_tile_from_chunk(&self, chunk_pos: &UVec3, tile_pos: &UVec2) -> Option<&Tile> {
        if self.invalid_chunk_pos(chunk_pos) {
            //println!("Invalid chunk pos");
            None
        } else {
            self.chunks[chunk_pos.x as usize][chunk_pos.y as usize][chunk_pos.z as usize].get_tile(tile_pos)
        }
    }

    pub fn get_tile_from_grid_pos(&self, grid_pos: &UVec2, layer: u32) -> Option<&Tile> {
        let (chunk_pos, tile_pos) = self.grid_pos_to_chunk_tile(grid_pos);
        self.get_tile_from_chunk(
            &UVec3::new(chunk_pos.x, chunk_pos.y, layer as u32),
            &tile_pos,
        )
    }

    fn grid_pos_to_chunk_tile(&self, grid_pos: &UVec2) -> (UVec2, UVec2) {
        (
            UVec2::new(
                grid_pos.x / self.chunk_size as u32,
                grid_pos.y / self.chunk_size as u32,
            ),
            UVec2::new(
                grid_pos.x % self.chunk_size as u32,
                grid_pos.y % self.chunk_size as u32,
            ),
        )
    }

    pub fn get_tile_mut(&mut self, chunk_pos: &UVec3, tile_pos: &UVec2) -> Option<&mut Tile> {
        if self.invalid_chunk_pos(chunk_pos) {
            None
        } else {
            let chunk =
                &mut self.chunks[chunk_pos.x as usize][chunk_pos.y as usize][chunk_pos.z as usize];
            chunk.get_tile_mut(tile_pos)
        }
    }

    pub fn get_tile_with_layer(
        &self,
        chunk_pos: &UVec2,
        layer: TilemapLayer,
        tile_pos: &UVec2,
    ) -> Option<&Tile> {
        self.get_tile_from_chunk(
            &UVec3::new(chunk_pos.x, chunk_pos.y, layer as u32),
            tile_pos,
        )
    }

    pub fn set_tile(
        &mut self,
        chunk_pos: &UVec3,
        tile_pos: &UVec2,
        new_tile: Tile,
    ) -> Result<(), String> {
        if self.invalid_chunk_pos(chunk_pos) {
            Err("Setting tile in out of bounds chunk pos".to_string())
        } else {
            let chunk =
                &mut self.chunks[chunk_pos.x as usize][chunk_pos.y as usize][chunk_pos.z as usize];
            chunk.set_tile(tile_pos, new_tile)
        }
    }

    pub fn set_tile_with_layer(
        &mut self,
        chunk_pos: &UVec2,
        layer: TilemapLayer,
        tile_pos: &UVec2,
        new_tile: Tile,
    ) -> Result<(), String> {
        self.set_tile(
            &UVec3::new(chunk_pos.x, chunk_pos.y, layer as u32),
            tile_pos,
            new_tile,
        )
    }

    pub fn real_to_chunk_and_tile(&self, real_pos: &Vec2, scaling_factor: f32) -> (UVec2, UVec2) {
        // Calculate chunk pos (scale down and divide by number of tiles per chunk and floor)
        let chunk_pos = (real_pos.clone() / (self.chunk_size as f32 * scaling_factor)).floor();
        // Calculate tile pos
        let chunk_real_pos = chunk_pos.clone() * (self.chunk_size as f32 * scaling_factor);
        // Offset by chunk real position then scale down and floor for tile position
        let tile_pos = ((real_pos.clone() - chunk_real_pos) / scaling_factor).floor();

        (
            UVec2::new(chunk_pos.x as u32, chunk_pos.y as u32),
            UVec2::new(tile_pos.x as u32, tile_pos.y as u32),
        )
    }

    pub fn real_to_grid_pos(&self, real_pos: &Vec2, scaling_factor: f32) -> UVec2 {
        let (chunk_pos, tile_pos) = self.real_to_chunk_and_tile(real_pos, scaling_factor);
        UVec2::new(
            tile_pos.x + chunk_pos.x * self.chunk_size as u32,
            tile_pos.y + chunk_pos.y * self.chunk_size as u32,
        )
    }

    fn invalid_chunk_pos(&self, chunk_pos: &UVec3) -> bool {
        chunk_pos.x >= self.dimensions.x
            || chunk_pos.y >= self.dimensions.y
            || chunk_pos.z >= self.dimensions.z
    }
}
