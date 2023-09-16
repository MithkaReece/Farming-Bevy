use bevy::prelude::*;

use crate::{
    components::chunk_component::EntityChunkMapping,
    resources::tilemap_resource::{GroundTilemap, ObjectTilemap},
};

pub fn sync_tile_visual(
    mut tiles: Query<(&mut TextureAtlasSprite, &mut Visibility)>,
    ground_tilemap: Res<GroundTilemap>,
    object_tilemap: Res<ObjectTilemap>,
    entity_chunk_map: Res<EntityChunkMapping>,
) {
    // Loop through all loaded chunks
    for ((chunk_x, chunk_y), chunk) in &ground_tilemap.tiles {
        // If a chunk is loaded
        if !chunk.is_loaded {
            continue;
        }
        // Get entities for each key (position)
        let entities_option = 
            entity_chunk_map.mapping.get(&(*chunk_x, *chunk_y));
        if entities_option.is_none() {
            continue;
        }
        let entities = entities_option.unwrap();

        let object_chunk_option = object_tilemap.tiles.get(&(*chunk_x, *chunk_y));
        // For each entity
        for ((x, y, z), &entity) in entities {
            // Get components
            if let Ok((mut sprite, mut visibility)) = 
                tiles.get_mut(entity) {
                // Get tile
                let mut tile_option = None;
                if *z == 0 {
                    //Ground
                    tile_option = chunk.tiles.get(&(*x, *y));
                } else if *z == 1 {
                    //Object
                    if object_chunk_option.is_none() {
                        continue;
                    }
                    let object_chunk = object_chunk_option.unwrap();
                    tile_option = object_chunk.tiles.get(&(*x, *y));
                }
                if tile_option.is_none() {
                    continue;
                }
                let tile = tile_option.unwrap();
                sprite.index = tile.get_index();
                *visibility = if tile.visible {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                }
            }
        }
    }
}
