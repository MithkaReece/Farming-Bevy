use bevy::prelude::*;

use crate::components::Player;

pub fn camera_follow(
    time: Res<Time>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
) {
    let mut camera_transform = camera.single_mut();
    let player_transform = player.single();
    
    // Smooth towards player
    let lerp_factor = 4.0;
    let target_position = player_transform.translation.clone();

    let position_difference = target_position - camera_transform.translation;

    camera_transform.translation +=
        (position_difference * lerp_factor * time.delta_seconds()).round();
    camera_transform.translation = camera_transform.translation.round();
}
