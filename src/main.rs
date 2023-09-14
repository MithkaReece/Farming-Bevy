mod components;
mod resources;
mod systems;

use bevy::{prelude::*, render::camera::ScalingMode};
use components::*;
use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Project Game".into(),
                        resolution: (1000.0, 700.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                character_movement,
                spawn_sheep,
                sheep_lifetime,
                sheep_target_setter,
                sheep_movement,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    // Load the sprite sheet image
    let texture_handle = asset_server.load("Thief_anim.png");
    // Create a TextureAtlas from the sprite sheet (with no padding and no offset)
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 30.0), 8, 5, None, None);
    // Add the TextureAtlas to the asset storage
    let atlas_handle = texture_atlases.add(texture_atlas);
    // Define the sprite for the specific frame you want to display
    let sprite_index = 0;

    // Spawn an entity with the selected sprite
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle,
            sprite: TextureAtlasSprite {
                index: sprite_index,
                ..Default::default()
            },
            transform: Transform::from_scale(Vec3::new(3.0, 3.0, 1.0)),
            ..Default::default()
        },
        Player { speed: 100.0 },
    ));
}
