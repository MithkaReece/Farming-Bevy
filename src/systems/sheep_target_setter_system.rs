use crate::{
    components::{chunk_component::CHUNK_SIZE, Target},
    resources::{tilemap_resource::GroundTilemap, ScalingFactor},
};
use bevy::prelude::*;

use super::MAP_POS;

struct BoxBounds {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

pub fn sheep_target_setter(
    mut sheeps: Query<(&Transform, &mut Target)>,
    ground_tilemap: Res<GroundTilemap>,
    scaling_factor: Res<ScalingFactor>,
) {
    let full_scaling_factor = scaling_factor.get_full_factor();
    let bounds = BoxBounds {
        min_x: MAP_POS.x,
        max_x: MAP_POS.x
            + (ground_tilemap.num_chunks_width * CHUNK_SIZE) as f32 * full_scaling_factor,
        min_y: MAP_POS.y,
        max_y: MAP_POS.y
            + (ground_tilemap.num_chunks_width * CHUNK_SIZE) as f32 * full_scaling_factor,
    };
    let min_distance = 1.0;

    for (transform, mut target) in &mut sheeps {
        // Close enough to target (set new)
        let distance = target
            .position
            .distance(Vec2::new(transform.translation.x, transform.translation.y));
        if distance < min_distance {
            // Make new target
            target.position = random_location_in_box(
                Vec2::new(transform.translation.x, transform.translation.y),
                10.0 * full_scaling_factor,
                &bounds,
            );
        }
    }
}

fn random_location_in_box(origin: Vec2, max_distance: f32, bounds: &BoxBounds) -> Vec2 {
    loop {
        let random_x = origin.x + rand::random::<f32>() * 2.0 * max_distance - max_distance;
        let random_y = origin.y + rand::random::<f32>() * 2.0 * max_distance - max_distance;

        if random_x >= bounds.min_x
            && random_x <= bounds.max_x
            && random_y >= bounds.min_y
            && random_y <= bounds.max_y
        {
            return Vec2::new(random_x, random_y);
        }
    }
}
