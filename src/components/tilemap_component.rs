use bevy::prelude::*;

use crate::{
    components::{chunk_component::Chunk, tile_component::Tile},
    config::layer_enum::TilemapLayer,
};

#[derive(Component)]
pub struct Tilemap {
    pub dimensions: UVec3,
    pub chunk_size: usize, //No. tiles of a chunk width/height
    pub chunks: Vec<Vec<Chunk>>,
    pub tiles: Vec<Vec<Vec<Option<Tile>>>>,
    pub tile_entities: Vec<Vec<Vec<Option<Entity>>>>,
    pub island_indexes: Vec<Vec<i32>>,
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
        let mut chunks = Vec::with_capacity(dimensions.x as usize);

        for x in 0..dimensions.x {
            let mut row = Vec::with_capacity(dimensions.y as usize);

            for y in 0..dimensions.y {
                row.push(Chunk::new(
                    UVec2::new(x * chunk_size as u32, y * chunk_size as u32),
                    chunk_size,
                ))
            }
            chunks.push(row);
        }

        let tile_dimension_x = dimensions.x as usize * chunk_size;
        let tile_dimension_y = dimensions.y as usize * chunk_size;

        Tilemap {
            dimensions,
            chunk_size,
            chunks,
            tiles: vec![
                vec![vec![None; dimensions.z as usize]; tile_dimension_y];
                tile_dimension_x
            ],
            tile_entities: vec![
                vec![vec![None; dimensions.z as usize]; tile_dimension_y];
                tile_dimension_x
            ],
            island_indexes: vec![vec![-1; tile_dimension_y]; tile_dimension_x],
        }
    }

    pub fn get_tile(&self, pos: &UVec3) -> Option<&Tile> {
        if self.invalid_pos(pos) {
            None
        } else {
            self.tiles[pos.x as usize][pos.y as usize][pos.z as usize].as_ref()
        }
    }

    pub fn get_tile_from_real(
        &self,
        real_pos: &Vec2,
        layer: TilemapLayer,
        scaling_factor: f32,
    ) -> Option<&Tile> {
        let grid_pos = self.real_to_grid_pos(real_pos, scaling_factor);
        self.get_tile(&UVec3::new(grid_pos.x, grid_pos.y, layer as u32))
    }

    pub fn get_tile_mut(&mut self, pos: &UVec3) -> Option<&mut Tile> {
        if self.invalid_pos(pos) {
            None
        } else {
            self.tiles[pos.x as usize][pos.y as usize][pos.z as usize].as_mut()
        }
    }

    pub fn get_tile_mut_from_real(
        &mut self,
        real_pos: &Vec2,
        layer: TilemapLayer,
        scaling_factor: f32,
    ) -> Option<&mut Tile> {
        let grid_pos = self.real_to_grid_pos(real_pos, scaling_factor);
        self.get_tile_mut(&UVec3::new(grid_pos.x, grid_pos.y, layer as u32))
    }

    pub fn set_tile(&mut self, pos: &UVec3, new_tile: Tile) -> Result<(), String> {
        if self.invalid_pos(pos) {
            Err("Setting tile in out of bounds chunk pos".to_string())
        } else {
            self.tiles[pos.x as usize][pos.y as usize][pos.z as usize] = Some(new_tile);
            Ok(())
        }
    }

    pub fn set_tile_from_real(
        &mut self,
        real_pos: &Vec2,
        layer: TilemapLayer,
        scaling_factor: f32,
        new_tile: Tile,
    ) -> Result<(), String> {
        let grid_pos = self.real_to_grid_pos(real_pos, scaling_factor);
        self.set_tile(&UVec3::new(grid_pos.x, grid_pos.y, layer as u32), new_tile)
    }

    fn invalid_pos(&self, pos: &UVec3) -> bool {
        pos.x >= self.dimensions.x * self.chunk_size as u32
            || pos.y >= self.dimensions.y * self.chunk_size as u32
            || pos.z >= self.dimensions.z
    }

    pub fn get_entity(&self, pos: &UVec3) -> Option<Entity> {
        if self.invalid_pos(pos) {
            println!("Tried to get an entity of invalid position");
            None
        } else {
            self.tile_entities[pos.x as usize][pos.y as usize][pos.z as usize]
        }
    }

    fn get_chunk(&self, pos: &UVec2) -> Option<&Chunk> {
        if self.invalid_chunk(pos) {
            None
        } else {
            Some(&self.chunks[pos.x as usize][pos.y as usize])
        }
    }

    fn get_chunk_mut(&mut self, pos: &UVec2) -> Option<&mut Chunk> {
        if self.invalid_chunk(pos) {
            None
        } else {
            Some(&mut self.chunks[pos.x as usize][pos.y as usize])
        }
    }

    fn invalid_chunk(&self, pos: &UVec2) -> bool {
        pos.x > self.dimensions.x || pos.y > self.dimensions.y
    }

    pub fn get_entity_from_real(
        &self,
        real_pos: &Vec2,
        layer: TilemapLayer,
        scaling_factor: f32,
    ) -> Option<Entity> {
        let grid_pos = self.real_to_grid_pos(real_pos, scaling_factor);
        self.get_entity(&UVec3::new(grid_pos.x, grid_pos.y, layer as u32))
    }

    pub fn set_entity(&mut self, pos: &UVec3, entity: Option<Entity>) {
        if self.invalid_pos(pos) {
            println!("Tried to set an entity of invalid position");
            return;
        } else {
            self.tile_entities[pos.x as usize][pos.y as usize][pos.z as usize] = entity;
        }
    }

    pub fn despawn_entity(&self, pos: &UVec3, commands: &mut Commands<'_, '_>) {
        if self.invalid_pos(pos) {
            println!("Tried to despawn an entity of invalid position");
            return;
        } else {
            let entity = self.tile_entities[pos.x as usize][pos.y as usize][pos.z as usize];
            match entity {
                Some(entity) => commands.entity(entity).despawn(),
                None => {
                    println!("Tried to unload chunk that was unloaded")
                }
            }
        }
    }

    pub fn real_to_grid_pos(&self, real_pos: &Vec2, scaling_factor: f32) -> UVec2 {
        UVec2::new(
            (real_pos.x as f32 / scaling_factor) as u32,
            (real_pos.y as f32 / scaling_factor) as u32,
        )
    }

    fn grid_to_real_pos(&self, grid_pos: &UVec3, full_scaling_factor: f32) -> Vec3 {
        Vec3::new(grid_pos.x as f32, grid_pos.y as f32, grid_pos.z as f32) * full_scaling_factor
    }

    pub fn load_chunk(
        &mut self,
        chunk_pos: &UVec2,
        tile_scaling_factor: f32,
        full_scaling_factor: f32,
        commands: &mut Commands<'_, '_>,
    ) {
        let chunk = match self.get_chunk(chunk_pos) {
            Some(chunk) => chunk,
            None => return,
        };

        if chunk.is_loaded {
            return;
        }

        let mut new_tile_entities = self.tile_entities.clone();

        for x in
            chunk.chunk_bottom_left_pos.x..chunk.chunk_bottom_left_pos.x + self.chunk_size as u32
        {
            for y in chunk.chunk_bottom_left_pos.y
                ..chunk.chunk_bottom_left_pos.y + self.chunk_size as u32
            {
                for z in 0..TilemapLayer::EndOfLayers as u32 {
                    let grid_pos = UVec3::new(x, y, z);
                    let tile = self.get_tile(&grid_pos);

                    let visibility = match tile {
                        Some(_) => Visibility::Inherited,
                        None => Visibility::Hidden,
                    };
                    let real_tile_pos = self.grid_to_real_pos(&grid_pos, full_scaling_factor);

                    let entity = commands
                        .spawn((
                            SpriteSheetBundle {
                                transform: Transform::from_xyz(
                                    real_tile_pos.x,
                                    real_tile_pos.y,
                                    real_tile_pos.z,
                                ) * Transform::from_scale(Vec3::splat(
                                    tile_scaling_factor,
                                )),
                                visibility,
                                ..Default::default()
                            },
                            Name::new(format!(
                                "({},{},{})",
                                real_tile_pos.x, real_tile_pos.y, real_tile_pos.z
                            )),
                        ))
                        .id();
                    new_tile_entities[x as usize][y as usize][z as usize] = Some(entity);
                }
            }
        }

        self.tile_entities = new_tile_entities;

        if let Some(chunk) = self.get_chunk_mut(chunk_pos) {
            chunk.is_loaded = true;
        }
    }

    pub fn unload_chunk(&mut self, chunk_pos: &UVec2, commands: &mut Commands<'_, '_>) {
        let chunk = match self.get_chunk(chunk_pos) {
            Some(chunk) => chunk,
            None => return,
        };

        if !chunk.is_loaded {
            return;
        }

        let mut new_tile_entities = self.tile_entities.clone();

        for x in
            chunk.chunk_bottom_left_pos.x..chunk.chunk_bottom_left_pos.x + self.chunk_size as u32
        {
            for y in chunk.chunk_bottom_left_pos.y
                ..chunk.chunk_bottom_left_pos.y + self.chunk_size as u32
            {
                for z in 0..TilemapLayer::EndOfLayers as u32 {
                    let entity = new_tile_entities[x as usize][y as usize][z as usize];
                    match entity {
                        Some(entity) => {
                            commands.entity(entity).despawn();
                            new_tile_entities[x as usize][y as usize][z as usize] = None;
                        }
                        None => {
                            println!("Tried to unload chunk that was unloaded")
                        }
                    }
                }
            }
        }

        self.tile_entities = new_tile_entities;

        if let Some(chunk) = self.get_chunk_mut(chunk_pos) {
            chunk.is_loaded = false;
        }
    }
}
