use bevy::prelude::*;

use crate::{
    components::{Tile, Tilemap},
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
    let chunk_size = tilemap.chunk_size as u32;

    for (_, row) in tilemap.chunks.iter_mut().enumerate() {
        for (_, col) in row.iter_mut().enumerate() {
            for (_, chunk) in col.iter_mut().enumerate() {
                if !chunk.is_loaded {
                    continue;
                }

                for row in 0..chunk_size as u32 {
                    for col in 0..chunk_size as u32 {
                        // Get entity of tile (TODO -> Should this error if not found)
                        let entity = match chunk.get_tile_entity(&UVec2::new(col, row)) {
                            Some(entity) => entity,
                            None => continue,
                        };
                        let tile = match chunk.get_tile(&UVec2::new(col, row)) {
                            Some(data ) => data,
                            None => {
                                // Delete entity as not in tilemap
                                continue;
                            }
                        };
   

                        // Sync data
                        if let Ok((mut atlas_handle, mut sprite, mut visibility)) =
                            tile_entities.get_mut(*entity)
                        {

                            sprite.index = tile.get_index();
                            // Updates visiblity
                            *visibility =  Visibility::Inherited;

                            // Map tile to spritesheet
                            let tile_info = tiles_info.get_tile(tile.get_group_name().as_str(),tile.get_type_name().as_str());
                            match tile_info {
                                Some(tile_data) => {
                                    *atlas_handle = match tile_data.spritesheet.as_str() {
                                        "farm_tilemap" => saved_atlas_handles.farm.clone(),
                                        "plants" => saved_atlas_handles.plants.clone(),
                                        _ =>  Default::default()
                                    };
                                }
                                None => { println!("can't find spritesheet for tile {:?}", tile)}
                            }
                        }
                    }
                }
            }
        }
    }
}
