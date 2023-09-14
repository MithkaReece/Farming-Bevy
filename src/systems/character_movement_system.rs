use bevy::prelude::*;

use crate::components::Player;

pub fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let mut movement_vec = Vec2::new(0.0, 0.0);

        if input.pressed(KeyCode::W) {
            movement_vec.y += 1.0;
        }
        if input.pressed(KeyCode::A) {
            movement_vec.x += -1.0;
        }
        if input.pressed(KeyCode::S) {
            movement_vec.y += -1.0;
        }
        if input.pressed(KeyCode::D) {
            movement_vec.x += 1.0;
        }
        if movement_vec.length() > 0.0 {
            movement_vec = movement_vec.normalize();
        }
        movement_vec *= player.speed * time.delta_seconds();

        transform.translation.x += movement_vec.x;
        transform.translation.y += movement_vec.y;
        transform.translation = transform.translation.round();
    }
}
