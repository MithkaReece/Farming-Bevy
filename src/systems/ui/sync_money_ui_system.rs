use bevy::prelude::*;

use crate::{components::MoneyText, resources::money_resource::Money};

pub fn sync_money_ui(mut money_text: Query<&mut Text, With<MoneyText>>, money: Res<Money>) {
    let mut money_text = money_text.single_mut();
    money_text.sections[0].value = format!("Money: {:.}", money.0);
}
