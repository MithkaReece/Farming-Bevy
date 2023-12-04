use bevy::prelude::*;

use crate::components::{Animal, Target};

/**
 * Move animal towards its current target
 * TODO: Flip animal based on direction
 * TODO: Animate animal based on moving
 */

pub fn animal_movement(mut animals: Query<(&mut Transform, &Animal, &Target)>, time: Res<Time>) {
    for (mut transform, animal, target) in &mut animals {
        let target_pos_grid = match (target.get_target()) {
            Some(pos) => pos,
            None => continue,
        };

        let target_pos_real = Vec3::new(
            target_pos_grid.x as f32,
            target_pos_grid.y as f32,
            transform.translation.z,
        );
        let direction = target_pos_real - transform.translation;

        let distance_to_move = animal.movement_speed * time.delta_seconds();

        if direction.length() <= distance_to_move {
            // Capable of moving to target in one move so snap to target
            transform.translation = target_pos_real
        } else {
            // Move towards target by distance_to_move amount
            transform.translation += distance_to_move
                * match direction.try_normalize() {
                    Some(dir) => dir,
                    None => return,
                };
        }
    }
}
