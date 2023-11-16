use bevy::prelude::*;

use crate::components::Inventory;

pub fn switch_selected_item(mut query: Query<&mut Inventory>, input: Res<Input<KeyCode>>) {
  let mut inv = query.single_mut();
  if input.just_pressed(KeyCode::Right) {
    inv.select_next_item_right();
  }else if input.just_pressed(KeyCode::Left) {
    inv.select_next_item_left();
  }
}