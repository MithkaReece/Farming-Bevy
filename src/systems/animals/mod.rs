pub mod animal_ai_system;
pub mod animal_stats_system;
pub mod update_animal_blackboard_system;
pub mod pathfinding;
pub mod animal_memory_system;

use self::animal_ai_system::*;
use self::animal_stats_system::*;
use self::update_animal_blackboard_system::*;
use self::animal_memory_system::*;

use bevy::prelude::*;

pub struct AnimalsPlugin;
impl Plugin for AnimalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animal_ai,
                animal_stats,
                update_animal_blackboard,
                animal_memory,
            ),
        );
    }
}
