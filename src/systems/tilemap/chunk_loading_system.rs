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
    let loading_radius = scaling_factor.full() * 30.0;

    // Check if ground tile chunk is visible
    // If so, check its loaded, if not then load, also load object at that chunk
    //If no visible, checks it unloaded,
    let player_transform = player.single();
    let player_position = Vec2::new(
        player_transform.translation.x + scaling_factor.full() / 2.0,
        player_transform.translation.y + scaling_factor.full() / 2.0,
    );

    let mut tilemap = tilemap.single_mut();

    for chunk_x in 0..tilemap.dimensions.x {
        for chunk_y in 0..tilemap.dimensions.y {
            let chunk_center = Vec2::new(chunk_x as f32, chunk_y as f32)
                * tilemap.chunk_size as f32
                * scaling_factor.full();

            let distance =
                (chunk_center.x - player_position.x).hypot(chunk_center.y - player_position.y);

            if distance <= loading_radius {
                tilemap.load_chunk(
                    &UVec2::new(chunk_x, chunk_y),
                    scaling_factor.factor,
                    scaling_factor.full(),
                    &mut commands,
                );
            } else {
                tilemap.unload_chunk(&UVec2::new(chunk_x, chunk_y), &mut commands)
            }
        }
    }
}
