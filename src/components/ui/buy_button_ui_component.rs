use bevy::prelude::*;

use crate::components::item_component::ItemType;

pub enum BuyItem {
    Item(ItemType),
}

#[derive(Component)]
pub struct BuyButtonUi {
    item_to_buy: BuyItem,
}
