use bevy::prelude::*;

use crate::components::{
    item_component::{ItemType},
    Inventory, PlantType,
};

pub fn buy_seeds(mut query: Query<&mut Inventory>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::R) {
        let mut inv = query.single_mut();
        inv.add(ItemType::Seed(PlantType::Pumpkin));
        // let count = inv.get_number(ItemType::Seed(SeedType::Pumpkin));
        // println!("{:?}", count);
    }
    if input.just_pressed(KeyCode::T) {
        let mut inv = query.single_mut();
        inv.add(ItemType::Seed(PlantType::Carrot));
    }
    if input.just_pressed(KeyCode::Y) {
        let mut inv = query.single_mut();
        inv.add(ItemType::Seed(PlantType::Potato));
    }
}
