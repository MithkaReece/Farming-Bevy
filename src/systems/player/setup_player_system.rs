use bevy::prelude::*;

use crate::{components::{Player, PlayerState}, resources::ScalingFactor};

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    scaling_factor: ResMut<ScalingFactor>,
) {
    // Load the sprite sheet image
    let texture_handle = asset_server.load("Thief_anim.png");
    // Create a TextureAtlas from the sprite sheet (with no padding and no offset)
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 30.0), 8, 5, Some(Vec2::new(0.0,1.0)), Some(Vec2::new(0.0,1.0)));
    // Add the TextureAtlas to the asset storage
    let atlas_handle = texture_atlases.add(texture_atlas);
    // Define the sprite for the specific frame you want to display
    let sprite_index = 1;

    // Spawn an entity with the selected sprite
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle,
            sprite: TextureAtlasSprite {
                index: sprite_index,
                ..Default::default()
            },
            transform: Transform::from_scale(Vec3::splat(scaling_factor.factor))
                * Transform::from_translation(Vec3::new(50.0, 50.0, 3.0)),
            ..Default::default()
        },
        Player { speed: 500.0, looking_location: Vec2::new(0.0, 0.0),
        current_state: PlayerState::Idle },
        Name::new("Player"),
    ));
}
