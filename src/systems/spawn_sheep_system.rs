use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    components::{Animal, AnimalType, Player, Sheep, Target, AnimalBT},
    resources::money_resource::Money, config::{animal_action_enum::{AnimalAction, self}, BehaviourTreeNode},
};

pub fn spawn_sheep(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    // When you press space
    if !input.just_pressed(KeyCode::F) {
        return;
    }
    let player_transform = player.single();

    if money.0 < 10.0 {
        return;
    }
    money.0 -= 10.0;
    info!("Spent £10 on a pig, remaining money: £{:?}", money.0);

    // Load the sprite sheet image
    let texture_handle = asset_server.load("animal_spritesheet.png");
    // Create a TextureAtlas from the sprite sheet (with no padding and no offset)
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        16,
        30,
        Some(Vec2::new(0.05, 0.05)),
        None,
    );
    // Add the TextureAtlas to the asset storage
    let atlas_handle = texture_atlases.add(texture_atlas);
    // Define the sprite for the specific frame you want to display
    let sprite_index = 336;

    // Spawn an entity with the selected sprite
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle,
            sprite: TextureAtlasSprite {
                index: sprite_index,
                ..Default::default()
            },
            transform: *player_transform,
            ..Default::default()
        },
        Animal {
            animal_type: AnimalType::Sheep,
            thirst: 40.0,
            hunger: 100.0,
            movement_speed: 30.0,
        },
        Target {
            position: Vec2::new(
                player_transform.translation.x,
                player_transform.translation.y,
            ),
        },
        AnimalBT(get_bt()),
    ));
}

use animal_action_enum::AnimalAction::*;
use BehaviourTreeNode::*;

macro_rules! Sequence {
    ( $( $child:expr ),* ) => {
        {
            let mut children = Vec::new();
            $( children.push($child); )*
            BehaviourTreeNode::Sequence {
                children,
                current_child_index: 0,
            }
        }
    };
}

macro_rules! Select {
    ( $( $child:expr ),* ) => {
        {
            let mut children = Vec::new();
            $( children.push($child); )*
            BehaviourTreeNode::Selector {
                children,
                current_child_index: 0,
            }
        }
    };
}

pub fn get_bt() -> BehaviourTreeNode<AnimalAction>{
    // Define behavior tree
    Sequence![
        Select![
            Inverter(Box::new(Action(Thirsty))),
            Select![
                Action(DrinkWater),
                Action(GoToWater),
                Action(LookForWater)
            ]
        ],
        Select![
            Inverter(Box::new(Action(Hungry))),
            Select![
                Action(EatFood),
                Action(GoToFood),
                Action(LookForFood)
            ]
        ],
        Select![
            Inverter(Box::new(Action(InHerd))),
            Select![
                Action(Breed),
                Action(MoveToHerd),
                Action(Wander)
            ]
        ],
        Action(Wander) 
    ]
}


// pub fn get_bt() -> bonsai_bt::BT<AnimalAction, String, serde_json::Value>{
//     let blackboard: HashMap<String, serde_json::Value> = HashMap::new();

//     let thirsty = bonsai_bt::Select(vec![
//         bonsai_bt::Invert(Box::new(bonsai_bt::Action(Thirsty))),
//         bonsai_bt::Select(vec![
//             bonsai_bt::Action(DrinkWater),
//             bonsai_bt::Action(GoToWater),
//             bonsai_bt::Action(LookForWater),
//         ]),
//     ]);

//     let food = bonsai_bt::Select(vec![
//         bonsai_bt::Invert(Box::new(bonsai_bt::Action(Hungry))),
//         bonsai_bt::Select(vec![
//             bonsai_bt::Action(EatFood),
//             bonsai_bt::Action(GoToFood),
//             bonsai_bt::Action(LookForFood),
//         ]),
//     ]);

//     let herd = bonsai_bt::Select(vec![
//         bonsai_bt::Invert(Box::new(bonsai_bt::Action(InHerd))),
//         bonsai_bt::Select(vec![
//             bonsai_bt::Action(Breed),
//             bonsai_bt::Action(MoveToHerd),
//             bonsai_bt::Action(Wander),
//         ]),
//     ]);

//     let root = bonsai_bt::Sequence(vec![thirsty, food, herd, bonsai_bt::Action(Wander)]);

//     bonsai_bt::BT::new(root, blackboard)
// }
