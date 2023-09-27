pub mod chunk_loading_system;
pub mod plant_growth_system;
pub mod setup_tilemap_system;
pub mod sync_tile_visual_system;
pub mod tile_hover_system;

use self::chunk_loading_system::*;
use self::plant_growth_system::*;
use self::setup_tilemap_system::*;
use self::sync_tile_visual_system::*;
use self::tile_hover_system::*;

use bevy::prelude::*;

pub struct TilemapPlugin;
impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_tilemap);
        app.add_systems(
            Update,
            (chunk_loading, sync_tile_visual, tile_hover, plant_growth),
        );
    }
}
