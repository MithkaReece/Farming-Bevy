use bevy::prelude::*;

use crate::{
    components::{memory_component::Memory, Animal, AnimalBT, Target, Tilemap},
    config::{
        animal_action_enum::{self, AnimalAction},
        Status,
    },
    resources::ScalingFactor,
};
use animal_action_enum::AnimalAction::*;
use Status::*;

use super::pathfinding::a_star;

/**
 * Animal AI make decision of current action
 *
 * Want to go to a particular location (pathfind) -> action when reached
 * Stop walking through colliding tiles
 *
 * Efficient way to remember places that are also accessible
 * So lets say you see a water tile, there is no path
 * I don't want to pathfind every second I see it, so I need some way to
 * label it as, inaccessible
 * This should be within the tile
 * In fact within tile generation, tiles should have an accessibility
 * Okey but what if its within an island, water could be accessible from an island
 * Okey so we need to divide the world in accessibility islands
 *
 * So after tilemap is generated. We need a accessibility pass
 * Algorithm:
 * Loop through tilemap (nested for loop for all tiles)
 * -1 means unset island, 0 means inaccessible
 * current_island_index = 1
 * If tile != collider && tile.island == -1
 *  Set all tile neighbours recursively (use queues) to current island index
 *  When exhausted
 *      current_island_index++
 * Do until all tiles have been checked (has tile.island)
 *
 */

pub fn animal_ai(
    mut animals: Query<(
        &mut Transform,
        &Animal,
        &mut Target,
        &mut Memory,
        &mut AnimalBT,
    )>,
    time: Res<Time>,
    scaling_factor: Res<ScalingFactor>,
    tilemap: Query<&Tilemap>,
) {
    let tilemap = tilemap.single();

    for (mut transform, animal, mut target, mut memory, mut bt) in &mut animals {
        let dt = time.delta_seconds_f64();

        let bt = &mut bt.0;

        let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let grid_pos = tilemap.real_to_grid_pos(&current_pos, scaling_factor.full());

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
                println!("Go to water");
                if let Some(_) = memory.top_water() {
                    if target.has_target(&current_pos) {
                        Running
                    } else {
                        println!("Water has been reached hopefully");
                        Failure
                    }
                } else {
                    println!("Go to water failed");
                    Failure
                }
            }
            LookForWater => {
                println!("Look for water");

                if let Some(water_pos) = memory.top_water() {
                    if let Some(path) = a_star(tilemap, &grid_pos, water_pos) {
                        target.path = path;
                    } else {
                        println!("Pathfind Err: {:?} to {:?}", grid_pos, water_pos);
                    }
                    // Reset tree as pathfind has been calculated
                    Failure
                } else {
                    if target.has_target(&current_pos) {
                        Running
                    } else {
                        println!("Set random target");
                        target.set_random_target(tilemap, scaling_factor.full(), &current_pos);
                        Failure
                    }
                }
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
                //println!("Look for food");
                Running
            }
            Breed => {
                //println!("Breed");
                Success
            }
            MoveToHerd => {
                //println!("Move to herd");
                Success
            }
            Wander => {
                if target.has_target(&current_pos) {
                    println!("Wander");
                    Running
                } else {
                    println!("Set random target");
                    target.set_random_target(tilemap, scaling_factor.full(), &current_pos);
                    Failure
                }
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
