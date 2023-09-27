use bevy::prelude::*;

use crate::{
    components::{Animal, AnimalBT, Target},
    config::{
        animal_action_enum::{self, AnimalAction},
        Status,
    },
};
use animal_action_enum::AnimalAction::*;
use Status::*;
pub fn animal_ai(
    mut animals: Query<(&mut Transform, &Animal, &Target, &mut AnimalBT)>,
    time: Res<Time>,
) {
    for (mut transform, animal, target, mut bt) in &mut animals {
        let dt = time.delta_seconds_f64();

        let bt = &mut bt.0;

        bt.execute(&mut |action| match action {
            DrinkWater => {
                // println!("Drink water");
                // if try_drink() {
                Failure
                // } else {
                //     (bonsai_bt::Failure, dt)
                // }
            }
            GoToWater => {
                // Needs memory setup (therefore herd setup)
                // println!("Go to water");
                Failure
            }
            LookForWater => {
                println!("Look for water");
                move_towards_target(
                    &mut transform,
                    &target,
                    animal.movement_speed,
                    time.delta_seconds(),
                );
                Running
            }
            EatFood => {
                // println!("Eat food");
                Failure
            }
            GoToFood => {
                // Needs memory setup (therefore herd setup)
                // println!("Go to food");
                Failure
            }
            LookForFood => {
                println!("Look for food");
                Running
            }
            Breed => {
                println!("Breed");
                Success
            }
            MoveToHerd => {
                println!("Move to herd");
                Success
            }
            Wander => {
                move_towards_target(
                    &mut transform,
                    &target,
                    animal.movement_speed,
                    time.delta_seconds(),
                );
                //println!("Wander");
                Running
            }
        });

        // let mut bt = bt.0.clone();
        // println!("{}", animal.thirst);
        // let e: bonsai_bt::Event = bonsai_bt::UpdateArgs { dt }.into();
        // bt.state.tick(&e, &mut |args| match *args.action {
        //     Thirsty => {
        //         if animal.thirst < 30.0 {
        //             println!("Thirsty");
        //             Success
        //         } else {
        //             println!("Not Thirsty");
        //             (bonsai_bt::Failure, dt)
        //         }
        //     }
        //     DrinkWater => {
        //         println!("Drink waters");
        //         // if try_drink() {
        //         Success
        //         // } else {
        //         //     (bonsai_bt::Failure, dt)
        //         // }
        //     }
        //     GoToWater => {
        //         println!("Go to water");
        //         Success
        //     }
        //     LookForWater => {
        //         println!("Look for food");
        //         Success
        //     }
        //     Hungry => {
        //         println!("Hungry");
        //         Success
        //     }
        //     EatFood => {
        //         println!("Eat food");
        //         Success
        //     }
        //     GoToFood => {
        //         println!("Go to food");
        //         Success
        //     }
        //     LookForFood => {
        //         println!("Look for food");
        //         Success
        //     }
        //     InHerd => {
        //         println!("In herd");
        //         Success
        //     }
        //     Breed => {
        //         println!("Breed");
        //         Success
        //     }
        //     MoveToHerd => {
        //         println!("Move to herd");
        //         Success
        //     }
        //     Wander => {
        //         move_towards_target(
        //             &mut transform,
        //             &target,
        //             animal.movement_speed,
        //             time.delta_seconds(),
        //         );
        //         println!("Wander");
        //         Success
        //     }
        //     _ => {
        //         println!("Something");
        //         Success
        //     }
        // });
    }
}

fn move_towards_target(
    transform: &mut Mut<'_, Transform>,
    target: &Target,
    movement_speed: f32,
    dt: f32,
) {
    // println!("Wander");
    let target_position = Vec3::new(
        target.position.x,
        target.position.y,
        transform.translation.z,
    );
    let direction = target_position - transform.translation;

    let distance_to_move = movement_speed * dt;

    if direction.length() <= distance_to_move {
        transform.translation = target_position
    } else {
        transform.translation += distance_to_move
            * match direction.try_normalize() {
                Some(dir) => dir,
                None => return,
            };
    }
}
