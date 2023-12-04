use bevy::prelude::*;

use crate::{components::{TileType, Tilemap}, config::layer_enum::TilemapLayer};

pub fn plant_growth(
    time: Res<Time>,
    mut tilemap: Query<&mut Tilemap>,
) {    
    let mut tilemap = tilemap.single_mut();

    for x in 0..tilemap.dimensions.x*tilemap.chunk_size as u32 {
        for y in 0..tilemap.dimensions.y*tilemap.chunk_size as u32 {
            for layer in 0..TilemapLayer::EndOfLayers as u32 {
                let tile = match tilemap.get_tile_mut(&UVec3::new(x,y,layer)) {
                    Some(tile) => tile,
                    None => continue
                };

                match tile.tile_type {
                    TileType::Plant(_, ref mut plant) => {
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
