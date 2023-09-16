use std::time::{Duration, Instant};

use bevy::prelude::*;

use crate::{components::TileType, resources::tilemap_resource::ObjectTilemap};

pub fn plant_growth(time: Res<Time>, mut object_tilemap: ResMut<ObjectTilemap>) {
    // Loop through all chunks
    for ((_, _), chunk) in &mut object_tilemap.tiles {
        for (_, tile) in &mut chunk.tiles {
            match tile.tile_type {
                TileType::Seed(_, ref mut plant) => {
                    plant.time_since_stage += time.delta();
                    //println!("{:?}", plant.time_since_stage);
                    if !plant.has_expired() {
                        continue;
                    }
                    if plant.stage >= plant.max_stage {
                        continue;
                    }
                    //
                    plant.time_since_stage = Duration::from_secs(0);
                    plant.stage += 1;
                }
                _ => {}
            }
        }
    }
}
