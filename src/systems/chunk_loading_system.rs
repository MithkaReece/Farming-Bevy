use bevy::prelude::*;

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
    for ((chunk_x, chunk_y), chunk) in &mut ground_tilemap.tiles {
        let chunk_center_x =
            chunk.position.x + (scaling_factor.get_full_factor() * CHUNK_SIZE as f32 / 2.0);
        let chunk_center_y =
            chunk.position.y + (scaling_factor.get_full_factor() * CHUNK_SIZE as f32 / 2.0);
        let distance =
            (chunk_center_x - player_position.x).hypot(chunk_center_y - player_position.y);

        if distance <= loading_radius {
            if !chunk.is_loaded {
                chunk.is_loaded = true;
                // Load in chunk
                let mut entities = vec![];

                for row in 0..CHUNK_SIZE {
                    for col in 0..CHUNK_SIZE {
                        let entity = commands
                            .spawn((SpriteSheetBundle {
                                texture_atlas: ground_atlas_handle.clone(),
                                sprite: TextureAtlasSprite {
                                    index: 161,
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(
                                    chunk.position.x
                                        + scaling_factor.get_full_factor() * col as f32,
                                    chunk.position.y
                                        + scaling_factor.get_full_factor() * row as f32,
                                    0.0,
                                ) * Transform::from_scale(Vec3::splat(
                                    scaling_factor.factor,
                                )),
                                ..Default::default()
                            },))
                            .id();
                        entities.push(entity);
                    }
                }
                // Save entites for unloading
                entity_chunk_map
                    .mapping
                    .insert((*chunk_x, *chunk_y), entities);
            }
        } else {
            if chunk.is_loaded {
                chunk.is_loaded = false;
                // Unload chunk
                if let Some(entities_to_unload) =
                    entity_chunk_map.mapping.get(&(*chunk_x, *chunk_y))
                {
                    for &entity in entities_to_unload {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}
