use bevy::prelude::*;

use crate::item_component::ItemType;

#[derive(Debug)]
pub struct ItemInfo {
    pub item_type: ItemType,
    pub counter: usize,
}
#[derive(Debug)]
pub enum ItemRef {
    None,
    Item(ItemInfo),
}

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<ItemRef>,
    pub selected_index: usize,
}

impl Inventory {
    pub fn new(size: usize) -> Self {
        let mut items = Vec::new();
        for _ in 0..size {
            items.push(ItemRef::None);
        }
        Self {
            items,
            selected_index: 0,
        }
    }

    pub fn add(&mut self, item_type: ItemType) -> bool {
        if let Some(item_ref_found) = self.items.iter_mut().find(|item_ref| match item_ref {
            ItemRef::None => false,
            ItemRef::Item(item_info) => item_type == item_info.item_type,
        }) {
            match item_ref_found {
                ItemRef::None => {
                    println!("inventory component.add should never happen");
                    false
                }
                ItemRef::Item(item_info) => {
                    item_info.counter += 1;
                    println!("Item added (mul)");
                    true
                }
            }
        } else {
            // TODO Create item (Visual will have to be made)

            // Put item into first empty spots
            for item in &mut self.items {
                match item {
                    ItemRef::None => {
                        *item = ItemRef::Item(ItemInfo {
                            item_type,
                            counter: 1,
                        });
                        println!("Item added");
                        return true;
                    }
                    _ => continue,
                }
            }
            println!("Full inventory");
            // Full inventory
            false
        }
    }

    pub fn remove(&mut self, item_type: ItemType) -> bool {
        if let Some((item_ref_found)) = self.items.iter_mut().find(|(item_ref)| match item_ref {
            ItemRef::None => false,
            ItemRef::Item(item_info) => item_type == item_info.item_type,
        }) {
            match item_ref_found {
                ItemRef::None => {
                    println!("inventory component.add should never happen");
                    false
                }
                ItemRef::Item(item_info) => {
                    if item_info.counter > 0 {
                        item_info.counter -= 1;
                        // Remove item when counter get to 0
                        if item_info.counter <= 0 {
                            *item_ref_found = ItemRef::None;
                        }
                        true
                    } else {
                        println!("Removing from and item that doesn't exist");
                        false
                    }
                }
            }
        } else {
            false
        }
    }

    pub fn get_number(&self, item_type: ItemType) -> usize {
        if let Some(item_ref_found) = self.items.iter().find(|item_ref| match item_ref {
            ItemRef::None => false,
            ItemRef::Item(item_info) => item_type == item_info.item_type,
        }) {
            match item_ref_found {
                ItemRef::None => {
                    println!("inventory component.add should never happen");
                    0
                }
                ItemRef::Item(item_info) => item_info.counter,
            }
        } else {
            0
        }
    }

    pub fn get_selected_item(&self) -> Option<ItemType> {
        if self.items.len() > self.selected_index {
            match &self.items[self.selected_index] {
                ItemRef::None => None,
                ItemRef::Item(item_info) => Some(item_info.item_type.clone()),
            }
        } else {
            None
        }
    }
}
