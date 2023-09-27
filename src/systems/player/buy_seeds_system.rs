use bevy::prelude::*;

use crate::components::{Inventory, item_component::{ItemType, SeedType}};

pub fn buy_seeds(mut query: Query<&mut Inventory>, input: Res<Input<KeyCode>>) {
  if input.just_pressed(KeyCode::R) {
      let mut inv = query.single_mut();
      inv.add(ItemType::Seed(SeedType::Pumpkin));
      let count = inv.get_number(ItemType::Seed(SeedType::Pumpkin));
      println!("{:?}", count);
  }
}