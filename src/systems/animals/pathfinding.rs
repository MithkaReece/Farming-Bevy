use bevy::prelude::*;

use crate::{components::Tilemap, config::layer_enum::TilemapLayer};

#[derive(Debug, PartialEq)]
struct Node {
    parent_index: i32,
    position: UVec2,
    cost_g: f32,
    cost_f: f32,
}

impl Node {
    fn get_cost(&self) -> f32{
        self.cost_g + self.cost_f
    }
}

impl PartialOrd for Node{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_cost().partial_cmp(&other.get_cost())
    }
}

struct SortedList<T> {
    items: Vec<T>,
}

impl<T: PartialOrd> SortedList<T> {
    pub fn new() -> Self {
        SortedList { items: Vec::new() }
    }

    pub fn add(&mut self, item: T) {
        let index = match self.items.binary_search_by(|existing_item| existing_item.partial_cmp(&item).unwrap()) {
            Ok(idx) | Err(idx) => idx,
        };
        self.items.insert(index,item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

pub fn a_star(tilemap: &Tilemap, start: &UVec2, target: &UVec2) -> Option<Vec<UVec2>>{
    let neighbour_directions: [IVec2; 8] = [
        IVec2 { x:1, y:0},
        IVec2 { x:1, y:1},
        IVec2 { x:0, y:1},
        IVec2 { x:-1, y:1},
        IVec2 { x:-1, y:0},
        IVec2 { x:-1, y:-1},
        IVec2 { x:0, y:-1},
        IVec2 { x:1, y:-1},
    ];
    
    let mut closedset = Vec::new();
    let mut openset = SortedList::new();
    openset.add(Node{ parent_index: -1, position: start.clone(), cost_g: 0.0, cost_f: 0.0});
    
    let mut path = Vec::new();

    while openset.len() > 0 {
        // (Use of unwrap) -> No checks needed as they are already don
        closedset.push(openset.pop().unwrap());
        let mut current_node = closedset.last().unwrap();
        
        // Trace path from end to start
        if current_node.position == *target {
            while current_node.position != *start || current_node.parent_index == -1 {
                path.push(current_node.position.clone());
                current_node = match closedset.get(current_node.parent_index as usize) {
                    Some(node) => node,
                    None => return None
                }
            }
        }

        //For neighbour not collider and not in closed set
        //Try all 8 directions
        for dir in neighbour_directions.iter() {
            let x = current_node.position.x as i32 + dir.x;
            let y = current_node.position.y as i32 + dir.y;
            if x < 0 || y < 0 { continue };
            
            let neighbour_position = UVec2::new(x as u32, y as u32);

            // Skip colliding neighbours as no route
            for layer in 0..TilemapLayer::EndOfLayers as u32 {
                match tilemap.get_tile_from_grid_pos(
                    &neighbour_position,
                    layer,
                ) {
                    Some(tile) => { if tile.has_collision { continue }},
                    None => { continue}
                };
            }

            // Skipped neighbours in closedset as already done
            if let Some(_) = closedset.iter().find(|item| item.position == neighbour_position){
                continue;
            }

 
            // Find in open_set
            if let Some(neighbour_node) = openset.items.iter_mut().find(|item| item.position == neighbour_position) {
                let cost = current_node.cost_g + 
                    heuristic_cost_estimate(&current_node.position,&neighbour_position);
                if cost < neighbour_node.cost_g { // If smaller cost is found, replace existing cost
                    neighbour_node.cost_g = cost;
                }
            }else{ // First time seeing this node
                let cost = current_node.cost_g + 
                    heuristic_cost_estimate(&current_node.position,&neighbour_position);
                let neighbour_node = Node{ parent_index: closedset.len() as i32, position: neighbour_position, cost_g: cost, 
                    cost_f: heuristic_cost_estimate(&neighbour_position, &target)};
                openset.add(neighbour_node);
            }

           
        }
    }

    Some(path)
}

fn heuristic_cost_estimate(pos_a: &UVec2, pos_b: &UVec2) -> f32{
    let delta_x = i32::abs(pos_a.x as i32 - pos_b.x as i32);
    let delta_y = i32::abs(pos_a.y as i32 - pos_b.y as i32);

    if delta_x > delta_y {
        (14 * delta_y + 10 * (delta_x - delta_y)) as f32
    }else{
        (14 * delta_x + 10 * (delta_y - delta_x)) as f32
    }
}