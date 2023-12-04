use bevy::prelude::*;

use crate::{
    components::{Player, Tilemap},
    config::layer_enum::TilemapLayer,
    resources::ScalingFactor,
};

/***
 * TODO: Maybe make a separate layer for highlights as atm its just ground
 *
 */

pub fn tile_hover(
    mut player: Query<&mut Player>,
    mut tile_entities: Query<&mut TextureAtlasSprite>,
    scaling_factor: ResMut<ScalingFactor>,
    mut tilemap: Query<&mut Tilemap>,
) {
    let tilemap = tilemap.single_mut();
    let mut player = player.single_mut();

    let old_pos = Vec2::new(
        player.previous_looking_location.x,
        player.previous_looking_location.y,
    );
    if let Some(old_entity) =
        tilemap.get_entity_from_real(&old_pos, TilemapLayer::Ground, scaling_factor.full())
    {
        if let Ok(mut sprite) = tile_entities.get_mut(old_entity) {
            sprite.color = Color::WHITE;
        }
    }

    let real_pos = Vec2::new(player.looking_location.x, player.looking_location.y);
    if let Some(new_entity) =
        tilemap.get_entity_from_real(&real_pos, TilemapLayer::Ground, scaling_factor.full())
    {
        if let Ok(mut sprite) = tile_entities.get_mut(new_entity) {
            sprite.color = Color::Rgba {
                red: 1.0,
                green: 0.9,
                blue: 0.9,
                alpha: 1.0,
            };
        }
    }

    player.previous_looking_location = player.looking_location.clone();
}
