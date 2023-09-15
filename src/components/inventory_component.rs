use bevy::prelude::*;

use crate::item_component::ItemType;

pub struct ItemRef {
    pub item_type: ItemType,
    pub counter: usize,
}

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<ItemRef>,
    pub selected_index: usize,
}

impl Inventory {
    pub fn add(&mut self, item_type: ItemType) {
        if let Some(item_ref_found) = self
            .items
            .iter_mut()
            .find(|item_ref| item_ref.item_type == item_type)
        {
            item_ref_found.counter += 1;
        } else {
            // TODO Create item (Visual will have to be made)

            // For now ill just add the ItemType
            self.items.push(ItemRef {
                item_type,
                counter: 1,
            })
        }
    }

    pub fn remove(&mut self, item_type: ItemType) {
        if let Some((index, item_ref_found)) = self
            .items
            .iter_mut()
            .enumerate()
            .find(|(_, item_ref)| item_ref.item_type == item_type)
        {
            if item_ref_found.counter > 0 {
                item_ref_found.counter -= 1;
                // Remove item when counter get to 0
                if item_ref_found.counter <= 0 {
                  self.items.remove(index);
                }
            } else {
                println!("Removing from and item that doesn't exist")
            }
        }
    }

    pub fn get_number(&self, item_type: ItemType) -> usize {
        if let Some(item_ref_found) = self
            .items
            .iter()
            .find(|item_ref| item_ref.item_type == item_type)
        {
            item_ref_found.counter
        } else {
            0
        }
    }

    pub fn get_selected_item(&self) -> Option<ItemType> {
      if self.items.len() > self.selected_index {
        Some(self.items[self.selected_index].item_type.clone())
      }else{
        None
      }
    }
}
