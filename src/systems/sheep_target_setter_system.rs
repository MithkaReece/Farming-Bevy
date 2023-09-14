use crate::components::Target;
use bevy::prelude::*;

struct BoxBounds {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

pub fn sheep_target_setter(mut sheeps: Query<(&Transform, &mut Target)>) {
    let bounds = BoxBounds {
        min_x: -100.0,
        max_x: 100.0,
        min_y: -40.0,
        max_y: 90.0,
    };
    let min_distance = 1.0;

    for (transform, mut target) in &mut sheeps {
        // Close enough to target (set new)
        let distance = target
            .position
            .distance(Vec2::new(transform.translation.x, transform.translation.y));
        if distance < min_distance {
            // Make new target
            target.position = random_location_in_box(&bounds);
        }
    }
}

fn random_location_in_box(bounds: &BoxBounds) -> Vec2 {
    let random_x = bounds.min_x + rand::random::<f32>() * (bounds.max_x - bounds.min_x);
    let random_y = bounds.min_y + rand::random::<f32>() * (bounds.max_y - bounds.min_y);
    Vec2::new(random_x, random_y)
}
