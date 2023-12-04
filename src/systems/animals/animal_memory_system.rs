use std::sync::WaitTimeoutResult;

use bevy::prelude::*;

use crate::{
    components::{memory_component::Memory, Animal, GroundType, Tile, TileType, Tilemap},
    config::layer_enum::TilemapLayer,
    resources::ScalingFactor,
};

pub fn animal_memory(
    mut animals: Query<(&Transform, &Animal, &mut Memory)>,
    time: Res<Time>,
    scaling_factor: Res<ScalingFactor>,
    tilemap: Query<&Tilemap>,
) {
    let tilemap = tilemap.single();

    for (transform, animal, mut memory) in &mut animals {
        let grid_pos = tilemap.real_to_grid_pos(
            &Vec2::new(transform.translation.x, transform.translation.y),
            scaling_factor.full(),
        );

        let condition = |tile: &Tile| -> bool {
            match tile.tile_type {
                TileType::Plant(_, _) => true,
                _ => false,
            }
        };

        let objects = scan_radius(
            tilemap,
            &grid_pos,
            &TilemapLayer::Object,
            animal.sight_distance,
            &condition,
        );

        for object in objects {
            memory.add_food(object);
        }

        let water_condition = |tile: &Tile| -> bool {
            match tile.tile_type {
                TileType::Ground(GroundType::Water) => true,
                _ => false,
            }
        };

        let water_positions = scan_radius(
            tilemap,
            &grid_pos,
            &TilemapLayer::Ground,
            animal.sight_distance,
            &water_condition,
        );
        for water in water_positions {
            memory.add_water(water);
        }
        //println!("{:?}", memory.water_memory);
    }
}

fn scan_radius<F>(
    tilemap: &Tilemap,
    start_pos: &UVec2,
    layer: &TilemapLayer,
    dist: u32,
    condition: &F,
) -> Vec<UVec2>
where
    F: Fn(&Tile) -> bool,
{
    let mut collection = Vec::new();

    // Start
    scan(&tilemap, &mut collection, start_pos, layer, condition);

    // Up-Right
    for delta_x in 0..dist as i32 {
        for delta_y in 1..=(dist as i32 - delta_x) {
            let x = start_pos.x as i32 + delta_x;
            let y = start_pos.y as i32 + delta_y;
            if x < 0 || y < 0 {
                continue;
            };
            scan(
                &tilemap,
                &mut collection,
                &UVec2::new(x as u32, y as u32),
                layer,
                condition,
            );
        }
    }

    // Right-Down
    for delta_x in 1..=dist as i32 {
        for delta_y in 0..(dist as i32 - (delta_x - 1)) {
            let x = start_pos.x as i32 + delta_x;
            let y = start_pos.y as i32 - delta_y;
            if x < 0 || y < 0 {
                continue;
            };
            scan(
                &tilemap,
                &mut collection,
                &UVec2::new(x as u32, y as u32),
                layer,
                condition,
            );
        }
    }

    // Down-Left
    for delta_x in 0..dist as i32 {
        for delta_y in 1..=(dist as i32 - delta_x) {
            let x = start_pos.x as i32 - delta_x;
            let y = start_pos.y as i32 - delta_y;
            if x < 0 || y < 0 {
                continue;
            };
            scan(
                &tilemap,
                &mut collection,
                &UVec2::new(x as u32, y as u32),
                layer,
                condition,
            );
        }
    }

    // Left-Up
    for delta_x in 1..=dist as i32 {
        for delta_y in 0..(dist as i32 - (delta_x - 1)) {
            let x = start_pos.x as i32 - delta_x;
            let y = start_pos.y as i32 + delta_y;
            if x < 0 || y < 0 {
                continue;
            };
            scan(
                &tilemap,
                &mut collection,
                &UVec2::new(x as u32, y as u32),
                layer,
                condition,
            );
        }
    }

    collection
}

fn scan<F>(
    tilemap: &Tilemap,
    collection: &mut Vec<UVec2>,
    pos: &UVec2,
    layer: &TilemapLayer,
    condition: &F,
) where
    F: Fn(&Tile) -> bool,
{
    if let Some(tile) = tilemap.get_tile(&UVec3::new(pos.x, pos.y, *layer as u32)) {
        if condition(tile) {
            collection.push(pos.clone());
        }
    }
}
