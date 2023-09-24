use crate::components::{Sheep, Target};
use bevy::prelude::*;

pub fn sheep_movement(mut sheeps: Query<(&mut Transform, &Sheep, &Target)>, time: Res<Time>) {
    for (mut transform, sheep, target) in &mut sheeps {
        let target_position = Vec3::new(
            target.position.x,
            target.position.y,
            transform.translation.z,
        );
        let direction = target_position - transform.translation;

        let distance_to_move = sheep.movement_speed * time.delta_seconds();

        if direction.length() <= distance_to_move {
            transform.translation = target_position
        } else {
            transform.translation += distance_to_move
                * match direction.try_normalize() {
                    Some(dir) => dir,
                    None => return,
                };
        }
    }
}
