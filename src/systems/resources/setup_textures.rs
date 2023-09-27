use bevy::prelude::*;

use crate::resources::texture_atlas_handle_resource::TextureAtlasHandle;

pub fn setup_textures(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut saved_atlas_handles: ResMut<TextureAtlasHandle>,
) {
    let farm_texture_handle = asset_server.load("farm_tilemap.png");
    let plant_texture_handle = asset_server.load("plants.png");

    let farm_texture_atlas = TextureAtlas::from_grid(
        farm_texture_handle,
        Vec2::new(16.0, 16.0),
        32,
        16,
        None,
        None,
    );

    let plant_texture_atlas = TextureAtlas::from_grid(
        plant_texture_handle,
        Vec2::new(16.0, 16.0),
        5,
        6,
        None,
        None,
    );

    saved_atlas_handles.farm = texture_atlases.add(farm_texture_atlas);
    saved_atlas_handles.plants = texture_atlases.add(plant_texture_atlas);
}
