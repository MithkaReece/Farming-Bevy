use bevy::prelude::*;

use crate::item_component::ItemType;

#[derive(Debug)]
pub struct ItemRef {
    pub item_type: ItemType,
    pub counter: usize,
}

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Option<ItemRef>>,
    pub selected_index: usize,
    visible: bool,
}

impl Inventory {
    pub fn new(size: usize) -> Self {
        let mut items = Vec::new();
        for _ in 0..size {
            items.push(None);
        }
        Self {
            items,
            selected_index: 0,
            visible: false,
        }
    }

    pub fn add(&mut self, item_type: ItemType) -> bool {
        if let Some(item_ref_found) = self.items.iter_mut().find(|item_ref| match item_ref {
            None => false,
            Some(item_info) => item_type == item_info.item_type,
        }) {
            match item_ref_found {
                None => {
                    println!("inventory component.add should never happen");
                    false
                }
                Some(item_info) => {
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
                    None => {
                        *item = Some(ItemRef {
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
            None => false,
            Some(item_info) => item_type == item_info.item_type,
        }) {
            match item_ref_found {
                None => {
                    println!("inventory component.add should never happen");
                    false
                }
                Some(item_info) => {
                    if item_info.counter > 0 {
                        item_info.counter -= 1;
                        // Remove item when counter get to 0
                        if item_info.counter <= 0 {
                            *item_ref_found = None;
                            self.select_next_item_right();
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

    pub fn remove_selected(&mut self) -> bool {
        if let Some(selected_item) = self.get_selected_item() {
            self.remove(selected_item.item_type)
        }else{
            false
        }
    }

    pub fn get_number(&self, item_type: ItemType) -> usize {
        if let Some(item_ref_found) = self.items.iter().find(|item_ref| match item_ref {
            None => false,
            Some(item_info) => item_type == item_info.item_type,
        }) {
            match item_ref_found {
                None => {
                    println!("inventory component.add should never happen");
                    0
                }
                Some(item_info) => item_info.counter,
            }
        } else {
            0
        }
    }

    pub fn get_selected_item(&self) -> Option<&ItemRef> {
        if self.selected_index < self.items.len() {
            return self.items[self.selected_index].as_ref()
        } else {
            None
        }
    }

    pub fn get_at(&self, index: usize) -> Option<&ItemRef>{
        if index < self.items.len() {
            self.items[index].as_ref()
        }else{
            None
        }
    }

    pub fn select_next_item_right(&mut self) {
        for i in (self.selected_index + 1)..self.items.len() {
            match self.items[i] {
                None => {}
                Some(_) => {
                    self.selected_index = i;
                    return;
                }
            }
        }
        for i in 0..(self.selected_index + 1) {
            match self.items[i] {
                None => {}
                Some(_) => {
                    self.selected_index = i;
                    return;
                }
            }
        }
    }

    pub fn select_next_item_left(&mut self) {
        for i in (0..self.selected_index).rev() {
            match self.items[i] {
                None => {}
                Some(_) => {
                    self.selected_index = i;
                    return;
                }
            }
        }
        for i in ((self.selected_index + 1)..self.items.len()).rev() {
            match self.items[i] {
                None => {}
                Some(_) => {
                    self.selected_index = i;
                    return;
                }
            }
        }
    }

    pub fn toggle_visibility(&mut self){
        self.visible = !self.visible;
    }

    pub fn get_visiblity(&self) -> bool{
        self.visible
    }

    
}
