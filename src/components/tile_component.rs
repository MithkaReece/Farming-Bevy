use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {
    pub unique_id: usize,
    pub sprite_index: usize,
}
