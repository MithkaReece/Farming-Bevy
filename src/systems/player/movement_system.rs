use bevy::prelude::*;

use crate::{
    components::Player,
    resources::{ScalingFactor, TilemapInfo},
};


pub fn character_movement(
    mut players: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    scaling_factor: Res<ScalingFactor>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut players {
        let mut direction_normalised = Vec2::new(0.0, 0.0);

        if input.pressed(KeyCode::W) {
            direction_normalised.y += 1.0;
        }
        if input.pressed(KeyCode::A) {
            direction_normalised.x += -1.0;
        }
        if input.pressed(KeyCode::S) {
            direction_normalised.y += -1.0;
        }
        if input.pressed(KeyCode::D) {
            direction_normalised.x += 1.0;
        }
        if direction_normalised.length() > 0.0 {
            direction_normalised = direction_normalised.normalize();
        }

        transform.translation.x += direction_normalised.x * player.speed * time.delta_seconds();
        transform.translation.y += direction_normalised.y * player.speed * time.delta_seconds();
        transform.translation = transform.translation.round();

        // Set looking direction
        if direction_normalised.length() > 0.0 {
            let dist = scaling_factor.get_full_factor() * 1.5;
            player.looking_location = Vec2::new(
                (transform.translation.x + scaling_factor.get_full_factor() / 2.0)
                    + direction_normalised.x * dist,
                transform.translation.y + direction_normalised.y * dist,
            );
            // println!("{:?}", player.looking_location);
            if direction_normalised.x > 0.0 || direction_normalised.x < 0.0 {
                let scale_x = scaling_factor.factor * direction_normalised.x
                    / f32::abs(direction_normalised.x);
                transform.scale = Vec3::new(scale_x, transform.scale.y, transform.scale.z);
            }
        }
    }
}
