use bevy::prelude::*;

use crate::{
    components::Tilemap,
    config::layer_enum::TilemapLayer,
    resources::{texture_atlas_handle_resource::TextureAtlasHandle, tile_info_resource::TilesData},
};

// If tile is type None this set visiblity to false, otherwise true

pub fn sync_tile_visual(
    mut tile_entities: Query<(
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
        &mut Visibility,
    )>,
    mut tilemap: Query<&mut Tilemap>,
    saved_atlas_handles: Res<TextureAtlasHandle>,
    tiles_info: Res<TilesData>,
) {
    let mut tilemap = tilemap.single_mut();

    for x in 0..tilemap.dimensions.x * tilemap.chunk_size as u32 {
        for y in 0..tilemap.dimensions.y * tilemap.chunk_size as u32 {
            for layer in 0..TilemapLayer::EndOfLayers as u32 {
                let entity = match tilemap.get_entity(&UVec3::new(x, y, layer)) {
                    Some(entity) => entity,
                    None => continue,
                };
                let tile = match tilemap.get_tile(&UVec3::new(x, y, layer)) {
                    Some(tile) => tile,
                    None => {
                        // Delete entity as not in tilemap
                        continue;
                    }
                };

                // Sync data
                if let Ok((mut atlas_handle, mut sprite, mut visibility)) =
                    tile_entities.get_mut(entity)
                {
                    // Updates visiblity
                    *visibility = Visibility::Inherited;

                    // Map tile to spritesheet
                    let tile_info = tiles_info.get_tile(
                        tile.tile_type.get_group_name().as_str(),
                        tile.tile_type.get_type_name().as_str(),
                    );
                    match tile_info {
                        Some(tile_data) => {
                            *atlas_handle = match tile_data.spritesheet.as_str() {
                                "farm_tilemap" => saved_atlas_handles.farm.clone(),
                                "plants" => saved_atlas_handles.plants.clone(),
                                _ => Default::default(),
                            };

                            sprite.index = tile.tile_type.apply_index(tile_data.sprite_index);
                        }
                        None => {
                            println!("can't find spritesheet for tile {:?}", tile)
                        }
                    }
                }
            }
        }
    }
}
