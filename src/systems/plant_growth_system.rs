use bevy::prelude::*;

use crate::components::{Plant, Tile};

pub fn plant_growth(time: Res<Time>, mut plants: Query<(&mut Plant, &mut Tile)>) {
    for (mut plant, mut tile) in &mut plants {
        plant.growth_counter += time.delta_seconds();
        if plant.growth_counter > plant.time_between_stages {
            plant.growth_counter = 0.0;
            if plant.stage < plant.max_stage {
                plant.stage += 1;
                tile.index_offset = plant.stage;
            }
        }
    }
}
