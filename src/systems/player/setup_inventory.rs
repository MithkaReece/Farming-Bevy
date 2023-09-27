use bevy::prelude::*;

use crate::components::Inventory;

pub fn setup_inventory(mut commands: Commands) {
    commands.spawn(Inventory { items: Vec::new(), selected_index: 0 });
}
