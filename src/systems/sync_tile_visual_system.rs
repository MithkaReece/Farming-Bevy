use bevy::prelude::*;

use crate::components::Tile;

pub fn sync_tile_visual(mut tiles: Query<(&mut TextureAtlasSprite, &mut Visibility, &Tile)>) {
    for (mut sprite, mut visibility, tile) in &mut tiles {
        sprite.index = tile.get_index();
        if tile.visible {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
        //TODO: Add a check where the TileType's spritesheet matches the atlas in sprite
    }
}
