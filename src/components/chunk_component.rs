use bevy::prelude::*;

use crate::components::tile_component::Tile;

#[derive(Component, Clone)]
pub struct Chunk {
    pub chunk_size: usize,
    pub tiles: Vec<Vec<Option<Tile>>>,
    pub is_loaded: bool,
    pub tile_entities: Vec<Vec<Option<Entity>>>,
}

impl Chunk {
    pub fn new(chunk_size: usize) -> Self {
        let tiles = vec![vec![None; chunk_size]; chunk_size];

        Chunk {
            chunk_size,
            tiles,
            is_loaded: false,
            tile_entities: vec![vec![None; chunk_size]; chunk_size],
        }
    }

    pub fn get_tile(&self, tile_pos: &UVec2) -> Option<&Tile> {
        if tile_pos.x >= self.chunk_size as u32 || tile_pos.y >= self.chunk_size as u32 {
            println!("Tried to get a tile outside the chunk");
            None
        } else {
            self.tiles[tile_pos.x as usize][tile_pos.y as usize].as_ref()
        }
    }

    pub fn get_tile_mut(&mut self, tile_pos: &UVec2) -> Option<&mut Tile> {
        if tile_pos.x >= self.chunk_size as u32 || tile_pos.y >= self.chunk_size as u32 {
            None
        } else {
            self.tiles[tile_pos.x as usize][tile_pos.y as usize].as_mut()
        }
    }

    pub fn set_tile(&mut self, tile_pos: &UVec2, new_tile: Tile) -> Result<(), String> {
        if tile_pos.x >= self.chunk_size as u32 || tile_pos.y >= self.chunk_size as u32 {
            Err("Setting tile in out of bounds tile pos".to_string())
        } else {
            self.tiles[tile_pos.x as usize][tile_pos.y as usize] = Some(new_tile);
            Ok(())
        }
    }

    pub fn get_tile_entity<'a>(&'a self, tile_pos: &UVec2) -> Option<&'a Entity> {
        if tile_pos.x >= self.chunk_size as u32 || tile_pos.y >= self.chunk_size as u32 {
            None
        } else {
            self.tile_entities[tile_pos.x as usize][tile_pos.y as usize].as_ref()
        }
    }

    pub fn load(
        &mut self,
        chunk_pos: Vec2,
        chunk_layer: usize,
        tile_scaling_factor: f32,
        full_scaling_factor: f32,
        commands: &mut Commands<'_, '_>,
    ) {
        if self.is_loaded {
            return;
        }

        let real_chunk_pos = chunk_pos * self.chunk_size as f32;

        for row in 0..self.chunk_size {
            for col in 0..self.chunk_size {
                let visibility = match self.get_tile(&UVec2::new(col as u32, row as u32)) {
                    Some(tile) => Visibility::Inherited,
                    None => Visibility::Hidden
                };

                let real_tile_pos = Vec3::new(
                    real_chunk_pos.x + col as f32,
                    real_chunk_pos.y + row as f32,
                    chunk_layer as f32,
                ) * full_scaling_factor;

                let entity = commands
                    .spawn((
                        SpriteSheetBundle {
                            transform: Transform::from_xyz(
                                real_tile_pos.x,
                                real_tile_pos.y,
                                real_tile_pos.z,
                            ) * Transform::from_scale(Vec3::splat(tile_scaling_factor)),
                            visibility,
                            ..Default::default()
                        },
                        Name::new(format!(
                            "({},{},{})",
                            real_tile_pos.x, real_tile_pos.y, chunk_layer
                        )),
                    ),)
                    .id();
                self.tile_entities[col][row] = Some(entity);
            }
        }
        self.is_loaded = true;
    }

    pub fn unload(&mut self, commands: &mut Commands<'_, '_>) {
        if !self.is_loaded {
            return;
        }

        for row in &self.tile_entities {
            for cell in row {
                match cell {
                    Some(entity) => commands.entity(*entity).despawn(),
                    None => {
                        println!("Tried to unload chunk that was unloaded")
                    }
                }
            }
        }
        self.is_loaded = false;
    }
}
