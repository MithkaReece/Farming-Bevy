pub mod buy_seeds_system;
pub mod fence_place_system;
pub mod harvest_plant_system;
pub mod hoe_ground_system;
pub mod character_movement_system;
pub mod plant_seed_system;
pub mod setup_inventory_system;
pub mod setup_player_system;
pub mod spawn_sheep_system;
pub mod switch_selected_item_system;

use self::buy_seeds_system::*;
use self::fence_place_system::*;
use self::harvest_plant_system::*;
use self::hoe_ground_system::*;
use self::character_movement_system::*;
use self::plant_seed_system::*;
use self::setup_inventory_system::*;
use self::setup_player_system::*;
use self::spawn_sheep_system::*;
use self::switch_selected_item_system::*;

use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_player, setup_inventory));
        app.add_systems(
            Update,
            (
                character_movement,
                plant_seed.before(hoe_ground),
                hoe_ground,
                harvest_plant,
                spawn_sheep,
                fence_place,
                buy_seeds,
                switch_selected_item,
            ),
        );
    }
}
