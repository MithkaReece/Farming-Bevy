use bevy::prelude::*;

use crate::{
    components::{Player, Tilemap},
    resources::ScalingFactor,
};

pub fn chunk_loading(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut tilemap: Query<&mut Tilemap>,
    scaling_factor: Res<ScalingFactor>,
) {
    let LOADING_RADIUS = scaling_factor.get_full_factor() * 30.0;

    // Check if ground tile chunk is visible
    // If so, check its loaded, if not then load, also load object at that chunk
    //If no visible, checks it unloaded,
    let player_transform = player.single();
    let player_position = Vec2::new(
        player_transform.translation.x + scaling_factor.get_full_factor() / 2.0,
        player_transform.translation.y + scaling_factor.get_full_factor() / 2.0,
    );

    let mut tilemap = tilemap.single_mut();

    for (chunk_x, row) in tilemap.chunks.iter_mut().enumerate() {
        for (chunk_y, col) in row.iter_mut().enumerate() {
            for (chunk_z, mut chunk) in col.iter_mut().enumerate() {
                let chunk_center = Vec2::new(chunk_x as f32, chunk_y as f32)
                    * chunk.chunk_size as f32
                    * scaling_factor.get_full_factor();

                let distance =
                    (chunk_center.x - player_position.x).hypot(chunk_center.y - player_position.y);
                if distance <= LOADING_RADIUS {
                    // Load chunk
                    chunk.load(
                        Vec2::new(chunk_x as f32, chunk_y as f32),
                        chunk_z,
                        scaling_factor.factor,
                        scaling_factor.get_full_factor(),
                        &mut commands,
                    );
                } else {
                    chunk.unload(&mut commands);
                }
            }
        }
    }
}
