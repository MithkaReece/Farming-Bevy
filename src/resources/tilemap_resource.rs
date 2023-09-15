use bevy::{prelude::*};

use crate::components::Tileref;

#[derive(Resource)]
pub struct Tilemap {
    pub tiles: Vec<Tileref>,
    pub width: usize,
    pub height: usize,
}
