use bevy::prelude::*;

use crate::config::{animal_action_enum::AnimalAction, BehaviourTreeNode};

#[derive(Component, Clone)]
pub struct AnimalBT(pub BehaviourTreeNode<AnimalAction>);
