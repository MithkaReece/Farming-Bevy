use bevy::prelude::*;

use crate::components::{Inventory, InventoryCell};

pub fn sync_inventory_ui(
    mut inventory_cells: Query<&mut Text, With<InventoryCell>>,
    inventory: Query<&Inventory>,
) {
    let inventory = inventory.single();
    for (i, mut inventory_cell) in &mut inventory_cells.iter_mut().enumerate() {
        if let Some(item) = inventory.get_at(i) {
            inventory_cell.sections[0].value =
                format!("{:?}, {:.}", item.item_type.get_name(), item.counter);
        } else {
            inventory_cell.sections[0].value = format!("Empty");
        }
    }
}
