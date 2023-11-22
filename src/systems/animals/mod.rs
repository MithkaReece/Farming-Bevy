pub mod animal_ai_systems;
pub mod animal_stats_system;
pub mod animal_target_setter_system;
pub mod update_animal_blackboard_system;
pub mod pathfinding;

use self::animal_ai_systems::*;
use self::animal_stats_system::*;
use self::animal_target_setter_system::*;
use self::update_animal_blackboard_system::*;

use bevy::prelude::*;

pub struct AnimalsPlugin;
impl Plugin for AnimalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animal_ai,
                animal_stats,
                animal_target_setter,
                update_animal_blackboard,
            ),
        );
    }
}
