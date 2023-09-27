use bevy::prelude::*;

use crate::{components::{Tile, Tilemap}, config::layer_enum::Layer};

pub fn plant_growth(
    time: Res<Time>,
    mut tilemap: Query<&mut Tilemap>,
) {    
    let mut tilemap = tilemap.single_mut();
    let chunk_size = tilemap.chunk_size as u32;

    let num_chunks_x = tilemap.chunks.len();
    let num_chunks_y = tilemap.chunks[0].len();
    let num_chunks_z = tilemap.chunks[0][0].len();

    for chunk_x in 0..num_chunks_x {
        for chunk_y in 0..num_chunks_y {
            for chunk_z in 0..num_chunks_z {
                if chunk_z != Layer::Object as usize {
                    continue;
                }

                for y in 0..chunk_size as u32 {
                    for x in 0..chunk_size as u32 {
                        let chunk_pos = &UVec3::new(chunk_x as u32,chunk_y as u32, chunk_z as u32);
                        let tile_pos = &UVec2::new(x,y);
                        let tile = match tilemap.get_tile_mut(chunk_pos, tile_pos) {
                            Some(tile) => tile,
                            None => continue
                        };

                        match tile {
                            Tile::Seed(_, ref mut plant) => {
                                plant.growth_timer.tick(time.delta());
                                //println!("{:?}", plant.time_since_stage);
                                if !plant.growth_timer.finished() {
                                    continue;
                                }
                                if plant.stage >= plant.max_stage {
                                    continue;
                                }
                                plant.growth_timer.reset();
                                plant.stage += 1;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
