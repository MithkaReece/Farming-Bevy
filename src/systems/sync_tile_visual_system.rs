use bevy::prelude::*;

use crate::{
    components::{Tile, Tilemap},
    resources::texture_atlas_handle_resource::TextureAtlasHandle,
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
                        // Get tile
                        let tile = match chunk.get_tile(&UVec2::new(col, row)) {
                            Some(tile) => tile,
                            None => continue,
                        };
                        // Get entity of tile
                        let entity = match chunk.get_tile_entity(&UVec2::new(col, row)) {
                            Some(entity) => entity,
                            None => continue,
                        };

                        // Sync data
                        if let Ok((mut atlas_handle, mut sprite, mut visibility)) =
                            tile_entities.get_mut(*entity)
                        {
                            sprite.index = tile.get_index();
                            *visibility = match tile {
                                Tile::None => Visibility::Hidden,
                                _ => Visibility::Inherited,
                            };

                            // Depending on state
                            let correct_atlas_handle = match tile {
                                Tile::Grass | Tile::Hoed => saved_atlas_handles.farm.clone(),
                                Tile::Seed(_, _) => saved_atlas_handles.plants.clone(),
                                _ => Default::default(),
                            };

                            // Check if atlas_handle is changed
                            let no_change_needed = *atlas_handle == correct_atlas_handle;
                            if no_change_needed {
                                continue;
                            }

                            // If different Set atlas_handle to clone of atlas handle in resources
                            *atlas_handle = correct_atlas_handle;
                        }
                    }
                }
            }
        }
    }

    // // Loop through all loaded chunks
    // for ((chunk_x, chunk_y), chunk) in &ground_tilemap.tiles {
    //     // If a chunk is loaded
    //     if !chunk.is_loaded {
    //         continue;
    //     }
    //     // Get entities for each key (position)
    //     let entities_option = entity_chunk_map.mapping.get(&(*chunk_x, *chunk_y));
    //     if entities_option.is_none() {
    //         continue;
    //     }
    //     let entities = entities_option.unwrap();

    //     let object_chunk_option = object_tilemap.tiles.get(&(*chunk_x, *chunk_y));
    //     // For each entity
    //     for ((x, y, z), &entity) in entities {
    //         // Get components
    //         if let Ok((mut sprite,
    //             mut visibility)) = tile_entities.get_mut(entity) {
    //             // Get tile
    //             let mut tile_option = None;
    //             if *z == 0 {
    //                 //Ground
    //                 tile_option = chunk.tiles.get(&(*x, *y));
    //             } else if *z == 1 {
    //                 //Object
    //                 if object_chunk_option.is_none() {
    //                     continue;
    //                 }
    //                 let object_chunk = object_chunk_option.unwrap();
    //                 tile_option = object_chunk.tiles.get(&(*x, *y));
    //             }
    //             if tile_option.is_none() {
    //                 continue;
    //             }
    //             let tile = tile_option.unwrap();
    //             sprite.index = tile.get_index();
    //             *visibility = if tile.visible {
    //                 Visibility::Inherited
    //             } else {
    //                 Visibility::Hidden
    //             }
    //         }
    //     }
    // }
}
