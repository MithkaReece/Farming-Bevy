use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Project Game".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLUE),
        },
        ..default()
    });
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
    commands.spawn(SpriteSheetBundle {
        texture_atlas: atlas_handle,
        sprite: TextureAtlasSprite {
            index: sprite_index,
            ..Default::default()
        },
        transform: Transform::from_scale(Vec3::new(5.0,5.0,1.0)),
        ..Default::default()
    });
}
