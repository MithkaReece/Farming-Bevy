use bevy::prelude::*;

use crate::{components::Sheep, resources::money_resource::Money};

pub fn sheep_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut sheeps: Query<(Entity, &mut Sheep)>,
    mut money: ResMut<Money>,
) {
    for (sheep_entity, mut sheep) in &mut sheeps {
        sheep.lifetime.tick(time.delta());

        // if sheep.lifetime.finished() {
        //     money.0 += 15.0;

        //     commands.entity(sheep_entity).despawn();

        //     info!("Pig sold for £15! Current Money: £{:?}", money.0);
        // }
    }
}
