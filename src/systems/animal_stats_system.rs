use bevy::prelude::*;

use crate::components::Animal;

pub fn animal_stats(mut animals: Query<&mut Animal>, time: Res<Time>) {
    const THIRST_SPEED: f32 = 1.0;
    const HUNER_SPEED: f32 = 0.1;
    for mut animal in &mut animals {
        animal.thirst -= THIRST_SPEED * time.delta_seconds();
        animal.hunger -= HUNER_SPEED * time.delta_seconds();
    }
}
