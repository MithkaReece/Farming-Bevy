use bevy::{prelude::*, utils::HashMap};

use crate::{
    components::{
        chunk_component::{EntityChunkMapping, CHUNK_SIZE},
        Player,
    },
    resources::{
        tilemap_resource::{GroundTilemap, ObjectTilemap},
        ScalingFactor,
    },
};

use super::MAP_POS;

pub fn chunk_loading(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut ground_tilemap: ResMut<GroundTilemap>,
    mut object_tilemap: ResMut<ObjectTilemap>,
    mut entity_chunk_map: ResMut<EntityChunkMapping>,
    scaling_factor: Res<ScalingFactor>,
) {
    // Check if ground tile chunk is visible
    // If so, check its loaded, if not then load, also load object at that chunk
    //If no visible, checks it unloaded,
    let player_transform = player.single();
    let player_position = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y - scaling_factor.get_full_factor() / 2.0,
    );

    let loading_radius = scaling_factor.get_full_factor() * 20.0;

    // Load ground spritesheet
    let ground_texture_handle = asset_server.load("farm_tilemap.png");
    let ground_texture_atlas = TextureAtlas::from_grid(
        ground_texture_handle,
        Vec2::new(16.0, 16.0),
        32,
        16,
        None,
        None,
    );
    let ground_atlas_handle = texture_atlases.add(ground_texture_atlas);

    // Load ground spritesheet
    let plant_texture_handle = asset_server.load("plants.png");
    let plant_texture_atlas = TextureAtlas::from_grid(
        plant_texture_handle,
        Vec2::new(16.0, 16.0),
        5,
        6,
        None,
        None,
    );
    let plant_atlas_handle = texture_atlases.add(plant_texture_atlas);

    for ((chunk_x, chunk_y), chunk) in &mut ground_tilemap.tiles {
        let chunk_center_x =
            chunk.position.x + (scaling_factor.get_full_factor() * CHUNK_SIZE as f32 / 2.0);
        let chunk_center_y =
            chunk.position.y + (scaling_factor.get_full_factor() * CHUNK_SIZE as f32 / 2.0);
        let distance =
            (chunk_center_x - player_position.x).hypot(chunk_center_y - player_position.y);

        if distance <= loading_radius {
            if chunk.is_loaded {
                break;
            }
            chunk.is_loaded = true;
            // Load in chunk
            let mut entities = HashMap::new();

            // Load ground tilemap
            for row in 0..CHUNK_SIZE {
                for col in 0..CHUNK_SIZE {
                    let pos = Vec3::new(
                        chunk.position.x + scaling_factor.get_full_factor() * col as f32,
                        chunk.position.y + scaling_factor.get_full_factor() * row as f32,
                        0.0,
                    );

                    // Get ground tile
                    let tile_option = chunk.tiles.get(&(pos.x as usize, pos.y as usize));
                    if tile_option.is_none() {
                        println!("Ground tile not found in tilemap when loading");
                        break;
                    }
                    let tile = tile_option.unwrap();
                    let visibility = if tile.visible {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    };

                    let entity = commands
                        .spawn((SpriteSheetBundle {
                            texture_atlas: ground_atlas_handle.clone(),
                            sprite: TextureAtlasSprite {
                                index: tile.get_index(),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(pos.x, pos.y, pos.z)
                                * Transform::from_scale(Vec3::splat(scaling_factor.factor)),
                            visibility,
                            ..Default::default()
                        },))
                        .id();
                    entities.insert((pos.x as usize, pos.y as usize, pos.z as usize), entity);
                }
            }

            // Load object tilemap
            for row in 0..CHUNK_SIZE {
                for col in 0..CHUNK_SIZE {
                    let object_chunk_option = object_tilemap.tiles.get(&(*chunk_x, *chunk_y));
                    if object_chunk_option.is_none() {
                        break;
                    }
                    let object_chunk = object_chunk_option.unwrap();

                    let pos = Vec3::new(
                        chunk.position.x + scaling_factor.get_full_factor() * col as f32,
                        chunk.position.y + scaling_factor.get_full_factor() * row as f32,
                        1.0,
                    );

                    // Get object tile
                    let tile_option = object_chunk
                        .tiles
                        .get(&(pos.x.floor() as usize, pos.y.floor() as usize));
                    if tile_option.is_none() {
                        // println!(
                        //     "Ground tile not found in tilemap when loading, at ({:?}, {:?})",
                        //     pos.x, pos.y
                        // );
                        break;
                    }
                    let tile = tile_option.unwrap();
                    let visibility = if tile.visible {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    };

                    let entity = commands
                        .spawn((SpriteSheetBundle {
                            texture_atlas: plant_atlas_handle.clone(),
                            sprite: TextureAtlasSprite {
                                index: tile.get_index(),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(pos.x, pos.y, pos.z)
                                * Transform::from_scale(Vec3::splat(scaling_factor.factor)),
                            visibility,
                            ..Default::default()
                        },))
                        .id();
                    entities.insert((pos.x as usize, pos.y as usize, pos.z as usize), entity);
                }
            }

            // Save entites for unloading
            entity_chunk_map
                .mapping
                .insert((*chunk_x, *chunk_y), entities);
        } else {
            if !chunk.is_loaded {
                break;
            }
            chunk.is_loaded = false;
            // Unload chunk
            if let Some(entities_to_unload) = entity_chunk_map.mapping.get(&(*chunk_x, *chunk_y)) {
                for (_, &entity) in entities_to_unload {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
