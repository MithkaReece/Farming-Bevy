use bevy::prelude::*;

use crate::{
    components::{Player, Tilemap},
    config::layer_enum::Layer,
    resources::ScalingFactor,
};


pub fn character_movement(
    mut players: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    scaling_factor: Res<ScalingFactor>,
    time: Res<Time>,
    tilemap: Query<&Tilemap>,
) {
    let tilemap = tilemap.single();

    for (mut transform, mut player) in &mut players {
        let mut movement_norm = Vec2::new(0.0, 0.0);

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

        let movement_vec = Vec2::new(movement_norm.x * player.speed * time.delta_seconds(),
        movement_norm.y * player.speed * time.delta_seconds());

        let current_pos = Vec2::new(transform.translation.x, transform.translation.y);

        if !check_collision(&current_pos,Vec2::new(movement_vec.x,movement_vec.y), 
        tilemap, scaling_factor.get_full_factor()) { // Both x and y
            transform.translation.x += movement_vec.x;
            transform.translation.y += movement_vec.y;        
            update_looking_direction(&movement_norm, &scaling_factor, &mut transform, &mut player);
        }else if !check_collision(&current_pos,Vec2::new(movement_vec.x,0.0), 
        tilemap, scaling_factor.get_full_factor()) { // Only x
            transform.translation.x += movement_vec.x; 
            update_looking_direction(&Vec2::new(movement_norm.x, 0.0), &scaling_factor, &mut transform, &mut player);
        }else if !check_collision(&current_pos,Vec2::new(0.0,movement_vec.y), 
        tilemap, scaling_factor.get_full_factor()) { // Only y
            transform.translation.y += movement_vec.y; 
            update_looking_direction(&Vec2::new(0.0, movement_norm.y), &scaling_factor, &mut transform, &mut player);
        }
    }
}

fn update_looking_direction(movement_vec: &Vec2, scaling_factor: &Res<'_, ScalingFactor>, transform: &mut Mut<'_, Transform>, player: &mut Mut<'_, Player>){
    if movement_vec.length() > 0.0 {
        let dist = scaling_factor.get_full_factor() * 1.5;
        player.looking_location = Vec2::new(
            (transform.translation.x)
                + movement_vec.x * dist,
            transform.translation.y + movement_vec.y * dist,
        );
        // Flip player depending on looking direction
        if movement_vec.x > 0.0 || movement_vec.x < 0.0 {
            let scale_x = scaling_factor.factor * movement_vec.x
                / f32::abs(movement_vec.x);
            transform.scale = Vec3::new(scale_x, transform.scale.y, transform.scale.z);
        }
    }
}

fn check_collision(current_pos: &Vec2, current_direction: Vec2, tilemap: &Tilemap, scaling_factor: f32, ) -> bool {
    let left_pos = current_pos.clone() + current_direction - Vec2::new(-scaling_factor/4.0, scaling_factor/2.0);
    
    let (left_chunk_pos, left_tile_pos) = tilemap.from_pos_no_layer(
        &left_pos,
        scaling_factor,
    );

    let right_pos = left_pos.clone() + Vec2::new(scaling_factor/2.0,0.0);
    let (right_chunk_pos, right_tile_pos) = tilemap.from_pos_no_layer(
        &right_pos,
        scaling_factor,
    );

    for i in 0..Layer::EndOfLayers as u32 {
        match tilemap.get_tile(
            &UVec3::new(left_chunk_pos.x, left_chunk_pos.y, i),
            &left_tile_pos,
        ) {
            Some(tile) => {
                if tile.has_collision {
                    return true
                }
            },
            None => {}
        };
        match tilemap.get_tile(
            &UVec3::new(right_chunk_pos.x, right_chunk_pos.y, i),
            &right_tile_pos,
        ) {
            Some(tile) => {
                if tile.has_collision {
                    return true
                }
            },
            None => {}
        };
    }
    false
}
