use bevy::prelude::*;

use crate::components::{Plant, Tile};

pub fn sync_plant_visual(mut tiles: Query<(&mut Plant, &mut Tile)>) {
    for (mut plant, mut tile) in &mut tiles {
        tile.index_offset = plant.stage;
    }
}
