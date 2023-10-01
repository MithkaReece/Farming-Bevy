use bevy::prelude::*;

use crate::components::{Inventory, SelectedItemText};

pub fn sync_selected_item_ui(
    mut selected_item_text: Query<&mut Text, With<SelectedItemText>>,
    inventory: Query<&Inventory>,
) {
    let mut selected_item_text = selected_item_text.single_mut();
    let inventory = inventory.single();
    if let Some(selected_item) = inventory.get_selected_item() {
        selected_item_text.sections[0].value = format!("Selected: {:?}", selected_item.item_type.get_name());
    } else {
        selected_item_text.sections[0].value = format!("Selected: _");
    }
}
