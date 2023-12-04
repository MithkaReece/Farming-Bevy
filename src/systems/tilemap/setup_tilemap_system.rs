use bevy::prelude::*;

use crate::{
    components::{GroundType, Tile, TileType, Tilemap},
    config::layer_enum::TilemapLayer,
    resources::TilemapInfo,
};

use noise::{NoiseFn, Perlin};

/**
Biomes
Default (grass with ponds)
*/

pub fn setup_tilemap(mut commands: Commands, tilemap_info: Res<TilemapInfo>) {
    let mut tilemap = Tilemap::new(tilemap_info.dimensions, tilemap_info.chunk_size);

    //TODO: In future load data about world generation in and use that

    let perlin = Perlin::new(1); // Output range (-1,1)
    let tilemap_tile_width = tilemap_info.dimensions.x as f64 * tilemap_info.chunk_size as f64;
    let tilemap_tile_height = tilemap_info.dimensions.y as f64 * tilemap_info.chunk_size as f64;

    for x in 0..tilemap.dimensions.x * tilemap.chunk_size as u32 {
        for y in 0..tilemap.dimensions.y * tilemap.chunk_size as u32 {
            let val = perlin.get([
                x as f64 / tilemap_tile_width,
                y as f64 / tilemap_tile_height,
            ]);

            let tile = if val > 0.0 {
                Tile {
                    tile_type: TileType::Ground(GroundType::Grass),
                    has_collision: false,
                }
            } else {
                Tile {
                    tile_type: TileType::Ground(GroundType::Water),
                    has_collision: true,
                }
            };

            match tilemap.set_tile(&UVec3::new(x, y, TilemapLayer::Ground as u32), tile) {
                Err(e) => println!("{e}"),
                Ok(()) => {}
            };
        }
    }

    commands.spawn(tilemap);
}

/**
 * This algorithm exhaustively separates tiles in islands separated
 * by colliding tiles to easily detect when something is pathfindable
 *
 * Done using a frontier search
 * Prioritising none colliding tiles.
 * Only island is complete we search past colliding tiles to find another island
 */

fn island_algorithm(tilemap: &mut Tilemap) {
    let tilemap_width = tilemap.dimensions.x * tilemap.chunk_size as u32;
    let tilemap_height = tilemap.dimensions.y * tilemap.chunk_size as u32;

    let mut island_indexes = tilemap.island_indexes.clone();

    let mut visited_tiles = vec![vec![false; tilemap_height as usize]; tilemap_width as usize];
    let mut collider_tiles = Vec::new();
    let mut open_tiles = Vec::new();

    let mut current_island_index = 1;

    let start_pos = UVec2::new(0, 0);

    let mut previous_was_collider;

    if has_collider(tilemap, &start_pos) {
        previous_was_collider = true;
        collider_tiles.push(start_pos);
    } else {
        open_tiles.push(start_pos);
        previous_was_collider = false;
    }

    while collider_tiles.len() > 0 || open_tiles.len() > 0 {
        match open_tiles.pop() {
            Some(pos) => {
                if visited_tiles[pos.x as usize][pos.y as usize] {
                    continue; // Already visited
                }
                // Mark as visited
                visited_tiles[pos.x as usize][pos.y as usize] = true;
                previous_was_collider = false;

                island_indexes[pos.x as usize][pos.y as usize] = current_island_index;

                // Queue neighbours to frontier
                if pos.x > 0 {
                    let neighbour_pos = UVec2::new(pos.x - 1, pos.y);
                    if !visited_tiles[neighbour_pos.x as usize][neighbour_pos.y as usize] {
                        if has_collider(tilemap, &neighbour_pos) {
                            collider_tiles.push(neighbour_pos)
                        } else {
                            open_tiles.push(neighbour_pos)
                        }
                    }
                }
                if pos.y > 0 {
                    let neighbour_pos = UVec2::new(pos.x, pos.y - 1);
                    if !visited_tiles[neighbour_pos.x as usize][neighbour_pos.y as usize] {
                        if has_collider(tilemap, &neighbour_pos) {
                            collider_tiles.push(neighbour_pos)
                        } else {
                            open_tiles.push(neighbour_pos)
                        }
                    }
                }
                if pos.x < tilemap_width - 1 {
                    let neighbour_pos = UVec2::new(pos.x + 1, pos.y);
                    if !visited_tiles[neighbour_pos.x as usize][neighbour_pos.y as usize] {
                        if has_collider(tilemap, &neighbour_pos) {
                            collider_tiles.push(neighbour_pos)
                        } else {
                            open_tiles.push(neighbour_pos)
                        }
                    }
                }
                if pos.y < tilemap_height - 1 {
                    let neighbour_pos = UVec2::new(pos.x, pos.y + 1);
                    if !visited_tiles[neighbour_pos.x as usize][neighbour_pos.y as usize] {
                        if has_collider(tilemap, &neighbour_pos) {
                            collider_tiles.push(neighbour_pos)
                        } else {
                            open_tiles.push(neighbour_pos)
                        }
                    }
                }
            }
            None => match collider_tiles.pop() {
                Some(pos) => {
                    if visited_tiles[pos.x as usize][pos.y as usize] {
                        continue; // Already visited
                    }
                    // Mark as visited
                    visited_tiles[pos.x as usize][pos.y as usize] = true;

                    // This only occurs when an island has been finished
                    if !previous_was_collider {
                        current_island_index += 1;
                    }

                    previous_was_collider = true;

                    // Queue neighbours to frontier
                    if pos.x > 0 {
                        let neighbour_pos = UVec2::new(pos.x - 1, pos.y);
                        if !visited_tiles[neighbour_pos.x as usize][neighbour_pos.y as usize] {
                            if has_collider(tilemap, &neighbour_pos) {
                                collider_tiles.push(neighbour_pos)
                            } else {
                                open_tiles.push(neighbour_pos)
                            }
                        }
                    }
                    if pos.y > 0 {
                        let neighbour_pos = UVec2::new(pos.x, pos.y - 1);
                        if !visited_tiles[neighbour_pos.x as usize][neighbour_pos.y as usize] {
                            if has_collider(tilemap, &neighbour_pos) {
                                collider_tiles.push(neighbour_pos)
                            } else {
                                open_tiles.push(neighbour_pos)
                            }
                        }
                    }
                    if pos.x < tilemap_width - 1 {
                        let neighbour_pos = UVec2::new(pos.x + 1, pos.y);
                        if !visited_tiles[neighbour_pos.x as usize][neighbour_pos.y as usize] {
                            if has_collider(tilemap, &neighbour_pos) {
                                collider_tiles.push(neighbour_pos)
                            } else {
                                open_tiles.push(neighbour_pos)
                            }
                        }
                    }
                    if pos.y < tilemap_height - 1 {
                        let neighbour_pos = UVec2::new(pos.x, pos.y + 1);
                        if !visited_tiles[neighbour_pos.x as usize][neighbour_pos.y as usize] {
                            if has_collider(tilemap, &neighbour_pos) {
                                collider_tiles.push(neighbour_pos)
                            } else {
                                open_tiles.push(neighbour_pos)
                            }
                        }
                    }
                }
                None => {
                    println!("Dunno why this while loop would have run to get here");
                    break;
                }
            },
        };
    }

    tilemap.island_indexes = island_indexes;
}

fn has_collider(tilemap: &Tilemap, pos: &UVec2) -> bool {
    for z in 0..TilemapLayer::EndOfLayers as u32 {
        if let Some(tile) = tilemap.get_tile(&UVec3::new(pos.x, pos.y, z)) {
            if tile.has_collision {
                return true;
            }
        };
    }
    false
}
