use bevy::prelude::*;

use crate::{components::{AnimalBT, Animal}, config::animal_action_enum::AnimalCondition};

pub fn update_animal_blackboard(mut animals: Query<(&mut AnimalBT, &Animal)>){
  for (mut bt, animal) in &mut animals {
    let bt = &mut bt.0;
    bt.blackboard.update_condition(&AnimalCondition::Thirsty, 
      if animal.thirst < 30.0 {
          println!("Thirsty");
          true
      } else {
          println!("Not Thirsty");
          false
      }
      )
  }
}