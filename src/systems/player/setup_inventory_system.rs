use bevy::prelude::*;

use crate::components::Inventory;

pub fn setup_inventory(mut commands: Commands) {
    commands.spawn(Inventory::new(10));
}
