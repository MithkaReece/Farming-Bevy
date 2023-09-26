use bevy::prelude::*;

use crate::config::{animal_action_enum::{AnimalAction, AnimalCondition}, BT};

#[derive(Component, Clone)]
pub struct AnimalBT(pub BT<AnimalAction, AnimalCondition>);
