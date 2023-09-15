use bevy::{ecs::query::QuerySingleError, prelude::*};

use crate::{
    components::{Player, Sheep, Target},
    resources::money_resource::Money,
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

    match player.get_single() {
        Ok(player_transform) => {
            if money.0 >= 10.0 {
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
                    None,
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
                    Sheep {
                        lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                        movement_speed: 30.0,
                    },
                    Target {
                        position: Vec2::new(
                            player_transform.translation.x,
                            player_transform.translation.y,
                        ),
                    },
                ));
            }
        }
        Err(QuerySingleError::NoEntities(_)) => {
            println!("Error: No player found!");
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            println!("Error: Multiple players found!");
        }
    }
}
