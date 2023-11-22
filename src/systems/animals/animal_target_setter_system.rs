use crate::{
    components::Target,
    resources::{ScalingFactor, TilemapInfo},
};
use bevy::prelude::*;

struct BoxBounds {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

pub fn animal_target_setter(
    mut animals: Query<(&Transform, &mut Target)>,
    scaling_factor: Res<ScalingFactor>,
    tilemap_info: Res<TilemapInfo>,
) {
    let full_scaling_factor = scaling_factor.get_full_factor();
    let bounds = BoxBounds {
        min_x: 0.0,
        max_x: (tilemap_info.dimensions.x * tilemap_info.chunk_size as u32) as f32
            * full_scaling_factor,
        min_y: 0.0,
        max_y: (tilemap_info.dimensions.y * tilemap_info.chunk_size as u32) as f32
            * full_scaling_factor,
    };
    let min_distance = 1.0;

    for (transform, mut target) in &mut animals {
        // Close enough to target (set new)
        let distance = target
            .random_pos.as_vec2()
            .distance(Vec2::new(transform.translation.x, transform.translation.y));
        if distance < min_distance {
            // Make new target
            target.random_pos = random_location_in_box(
                Vec2::new(transform.translation.x, transform.translation.y),
                10.0 * full_scaling_factor,
                &bounds,
            );
        }
    }
}

fn random_location_in_box(origin: Vec2, max_distance: f32, bounds: &BoxBounds) -> UVec2 {
    loop {
        let random_x = origin.x + rand::random::<f32>() * 2.0 * max_distance - max_distance;
        let random_y = origin.y + rand::random::<f32>() * 2.0 * max_distance - max_distance;

        if random_x >= bounds.min_x
            && random_x <= bounds.max_x
            && random_y >= bounds.min_y
            && random_y <= bounds.max_y
        {
            return UVec2::new(random_x as u32, random_y as u32);
        }
    }
}
