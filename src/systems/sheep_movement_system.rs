use crate::components::{Sheep, Target};
use bevy::prelude::*;

pub fn sheep_movement(mut sheeps: Query<(&mut Transform, &Sheep, &Target)>, 
time: Res<Time>) {
    for (mut transform, sheep, target) in &mut sheeps {
        let mut target_direction = Vec2::new(
            target.position.x - transform.translation.x,
            target.position.y - transform.translation.y,
        );
        target_direction = target_direction.normalize();
        // Move sheep
        transform.translation.x += target_direction.x * sheep.movement_speed * time.delta_seconds();
        transform.translation.y += target_direction.y * sheep.movement_speed * time.delta_seconds();
    }
}
