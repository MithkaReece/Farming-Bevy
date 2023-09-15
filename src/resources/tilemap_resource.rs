use bevy::prelude::*;

use crate::components::Tileref;

#[derive(Resource)]
pub struct GroundTilemap {
    pub tiles: Vec<Tileref>,
    pub width: usize,
    pub height: usize,
}

#[derive(Resource)]
pub struct ObjectTilemap {
    pub tiles: Vec<Tileref>,
    pub width: usize,
    pub height: usize,
}
