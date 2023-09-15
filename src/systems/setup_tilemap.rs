use bevy::prelude::*;

use crate::{
    components::{Tile, Tileref},
    resources::{tilemap_resource::Tilemap, ScalingFactor},
};

pub fn setup_tilemap(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut tilemap: ResMut<Tilemap>,
    scaling_factor: ResMut<ScalingFactor>
) {
    // Load the sprite sheet image
    let texture_handle = asset_server.load("farm_tilemap.png");
    println!("{:?}", texture_handle);

    // Create a TextureAtlas from the sprite sheet (with no padding and no offset)
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 32, 16, None, None);

    // Add the TextureAtlas to the asset storage
    let atlas_handle = texture_atlases.add(texture_atlas);

    let map_width = 10;
    let map_height = 10;
    tilemap.width = map_width;
    tilemap.height = map_height;


    let mut tile_id = 0;
    // Add tiles to the tilemap
    for row in 0..map_height {
        for col in 0..map_width {
            let index = 32 * 5 + 1;
            // Figure out position
            let pos = Vec2::new(
                (scaling_factor.factor * scaling_factor.pixel_factor as f32 * col as f32).round(),
                (scaling_factor.factor * scaling_factor.pixel_factor as f32 * row as f32).round(),
            );
            // Create tile entity (sprite & tile components)
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
                        index,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(pos.x, pos.y, -1.0)
                        * Transform::from_scale(Vec3::splat(scaling_factor.factor)),
                    ..Default::default()
                },
                Tile {
                    unique_id: tile_id,
                    sprite_index: index,
                },
            ));
            // Track tile in tilemap
            tilemap.tiles.push(Tileref {
                position: pos,
                unique_id: tile_id,
            });
            // Increment identifer
            tile_id += 1;
        }
    }
}
