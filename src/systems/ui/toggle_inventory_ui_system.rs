use bevy::prelude::*;

use crate::components::Inventory;


pub fn toggle_inventory_ui(
    mut inventory: Query<&mut Inventory>,
    input: Res<Input<KeyCode>>
) {
    if input.just_pressed(KeyCode::E){
        let mut inventory = inventory.single_mut();
        inventory.toggle_visibility();
    }
}
