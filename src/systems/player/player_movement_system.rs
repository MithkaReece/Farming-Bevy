use bevy::prelude::*;

use crate::{
    components::{Player, Tilemap},
    config::layer_enum::TilemapLayer,
    resources::ScalingFactor,
    systems::pathfinding::a_star,
};

pub fn player_movement(
    mut players: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    scaling_factor: Res<ScalingFactor>,
    time: Res<Time>,
    tilemap: Query<&Tilemap>,
) {
    let tilemap = tilemap.single();

    for (mut transform, mut player) in &mut players {
        let mut movement_norm = Vec2::new(0.0, 0.0);

        if input.pressed(KeyCode::K) {
            let grid_pos = tilemap.real_to_grid_pos(
                &Vec2::new(transform.translation.x, transform.translation.y),
                scaling_factor.full(),
            );
            if let Some(mut path) = a_star(tilemap, &grid_pos, &UVec2::new(1, 1)) {
                println!("Start: {:?}", grid_pos);
                path.reverse();
                println!("{:?}", path);
            } else {
                println!("No path found");
            }
        }

        if input.pressed(KeyCode::W) {
            movement_norm.y += 1.0;
        }
        if input.pressed(KeyCode::A) {
            movement_norm.x += -1.0;
        }
        if input.pressed(KeyCode::S) {
            movement_norm.y += -1.0;
        }
        if input.pressed(KeyCode::D) {
            movement_norm.x += 1.0;
        }
        if movement_norm.length() > 0.0 {
            movement_norm = movement_norm.normalize();
        }

        let movement_vec = Vec2::new(
            movement_norm.x * player.speed * time.delta_seconds(),
            movement_norm.y * player.speed * time.delta_seconds(),
        );

        let current_pos = Vec2::new(transform.translation.x, transform.translation.y);

        if !check_collision(
            &current_pos,
            Vec2::new(movement_vec.x, movement_vec.y),
            tilemap,
            scaling_factor.full(),
        ) {
            // Both x and y
            transform.translation.x += movement_vec.x;
            transform.translation.y += movement_vec.y;
            update_looking_direction(&movement_norm, &scaling_factor, &mut transform, &mut player);
        } else if !check_collision(
            &current_pos,
            Vec2::new(movement_vec.x, 0.0),
            tilemap,
            scaling_factor.full(),
        ) {
            // Only x
            transform.translation.x += movement_vec.x;
            update_looking_direction(
                &Vec2::new(movement_norm.x, 0.0),
                &scaling_factor,
                &mut transform,
                &mut player,
            );
        } else if !check_collision(
            &current_pos,
            Vec2::new(0.0, movement_vec.y),
            tilemap,
            scaling_factor.full(),
        ) {
            // Only y
            transform.translation.y += movement_vec.y;
            update_looking_direction(
                &Vec2::new(0.0, movement_norm.y),
                &scaling_factor,
                &mut transform,
                &mut player,
            );
        }
    }
}

fn update_looking_direction(
    movement_vec: &Vec2,
    scaling_factor: &Res<'_, ScalingFactor>,
    transform: &mut Mut<'_, Transform>,
    player: &mut Mut<'_, Player>,
) {
    if movement_vec.length() > 0.0 {
        let dist = scaling_factor.full() * 1.5;
        player.looking_location = Vec2::new(
            (transform.translation.x) + movement_vec.x * dist,
            transform.translation.y + movement_vec.y * dist,
        );
        // Flip player depending on looking direction
        if movement_vec.x > 0.0 || movement_vec.x < 0.0 {
            let scale_x = scaling_factor.factor * movement_vec.x / f32::abs(movement_vec.x);
            transform.scale = Vec3::new(scale_x, transform.scale.y, transform.scale.z);
        }
    }
}

// TODO: May need to add snapping to collision edges
fn check_collision(
    current_pos: &Vec2,
    current_direction: Vec2,
    tilemap: &Tilemap,
    scaling_factor: f32,
) -> bool {
    let left_pos = current_pos.clone()
        + Vec2::new(0.22 * scaling_factor, -0.35 * scaling_factor)
        + current_direction;
    if left_pos.x <= 0.0 || left_pos.y <= 0.0 {
        return true;
    } // Checking for chunk edges

    let right_pos = current_pos.clone()
        + Vec2::new(0.78 * scaling_factor, -0.35 * scaling_factor)
        + current_direction;

    // Checking for chunk edges
    let mut nothing_left = true;
    let mut nothing_right = true;

    for i in 0..TilemapLayer::EndOfLayers as u32 {
        if let Some(tile) = tilemap.get_tile_from_real(&left_pos, i.into(), scaling_factor) {
            nothing_left = false;
            if tile.has_collision {
                return true;
            }
        };
        if let Some(tile) = tilemap.get_tile_from_real(&right_pos, i.into(), scaling_factor) {
            nothing_right = false;
            if tile.has_collision {
                return true;
            }
        };
    }
    nothing_left || nothing_right
}
