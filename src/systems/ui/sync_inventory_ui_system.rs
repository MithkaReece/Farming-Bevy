use bevy::prelude::*;

use crate::components::{Inventory, InventoryCell, InventoryUi};

pub fn sync_inventory_ui(
    mut inventory_visibility: Query<&mut Visibility, With<InventoryUi>>,
    mut inventory_cells: Query<&mut Text, With<InventoryCell>>,
    inventory: Query<&Inventory>,
) {
    
    let inventory = inventory.single();

    if !inventory.get_visiblity() {

    }
    let mut inventory_visibility = inventory_visibility.single_mut();
    let res = inventory_visibility.set(Box::new(if inventory.get_visiblity() {
        Visibility::Inherited.clone()
    }else{
        Visibility::Hidden.clone()
    }));
    match res{
        Err(e) => {println!("Inventory sync error => {:?}",e)}
        Ok(_)=>{}
    }


    // Sync cell inventory
    for (i, mut inventory_cell) in &mut inventory_cells.iter_mut().enumerate() {
        if let Some(item) = inventory.get_at(i) {
            inventory_cell.sections[0].value =
                format!("{:?}, {:.}", item.item_type.get_name(), item.counter);
        } else {
            inventory_cell.sections[0].value = format!("Empty");
        }
    }
}
