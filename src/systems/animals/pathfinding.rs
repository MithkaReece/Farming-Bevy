use bevy::prelude::*;

use crate::{components::Tilemap, config::layer_enum::TilemapLayer};

#[derive(Debug, PartialEq)]
struct Node {
    parent_index: i32,
    position: UVec2,
    cost_g: f32,
    cost_h: f32,
}

impl Node {
    fn get_cost(&self) -> f32{
        self.cost_g + self.cost_h
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
        let index = match self.items.binary_search_by(|existing_item| item.partial_cmp(&existing_item).unwrap()) {
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

// TODO - Optimise to use less nodes which will also solve the diagonal problem
pub fn a_star(tilemap: &Tilemap, start: &UVec2, target: &UVec2) -> Option<Vec<UVec2>>{
    // Can easily change to include diagonals
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
    openset.add(Node{ parent_index: -1, position: start.clone(), cost_g: 0.0, cost_h: 0.0});
    
    let mut path = Vec::new();

    let mut iterations = 0;

    while openset.len() > 0 {
        iterations += 1;

        // (Use of unwrap) -> No checks needed as they are already done
        closedset.push(openset.pop().unwrap());
        let mut current_node = closedset.last().unwrap();

        if iterations < 3 {
            //println!("{:?}", current_node);
        }
        //println!("{:?}, {:?}", current_node.position, current_node.get_cost());
        
        // Trace path from end to start
        if current_node.position == *target {
            let mut max_iterations = 1000;
            while current_node.position != *start && current_node.parent_index != -1 {
                max_iterations-=1;
                if max_iterations < 0 { return None }
                path.push(current_node.position.clone());
                //println!("=>{:?}", current_node);
                current_node = match closedset.get(current_node.parent_index as usize) {
                    Some(node) => node,
                    None => return None
                }
            }
            path.push(current_node.position.clone());
            //println!("=>{:?}", current_node);
            return Some(path);
        }

        //For neighbour not collider and not in closed set
        //Try all 8 directions
        for dir in neighbour_directions.iter() {
            let x = current_node.position.x as i32 + dir.x;
            let y = current_node.position.y as i32 + dir.y;
            if x < 0 || y < 0 { continue };
            
            let neighbour_position = UVec2::new(x as u32, y as u32);

            if check_collision(tilemap, &neighbour_position) { continue;}
            if dir.x != 0 && dir.y != 0 { // Diagonal - check either side is not collision so walkable
                if check_collision(tilemap, &UVec2::new(current_node.position.x, neighbour_position.y)) 
                    || check_collision(tilemap, &UVec2::new(neighbour_position.x, current_node.position.y)) {
                        continue;
                    }
            }

            // Skipped neighbours in closedset as already done
            if let Some(_) = closedset.iter().find(|item| item.position == neighbour_position){
                continue;
            }

 
            // Find in open_set
            if let Some(neighbour_node) = openset.items.iter_mut().find(|item| item.position == neighbour_position) {
                let cost = current_node.cost_g + 
                    heuristic_cost_estimate(&current_node.position,&neighbour_position);
                    //println!("{:?} < {:?}", cost, neighbour_node.cost_g);
                if cost < neighbour_node.cost_g { // If smaller cost is found, replace existing cost
                    neighbour_node.cost_g = cost;
                    if let Some(parent_index) = closedset.iter().position(|item| item.position == current_node.position) {
                        neighbour_node.parent_index = parent_index as i32;
                    }else{
                        println!("This part of the pathfinding should never happen");
                        return None;
                    }
                }
            }else{ // First time seeing this node
                let cost = current_node.cost_g + 
                    heuristic_cost_estimate(&current_node.position,&neighbour_position);
                if let Some(parent_index) = closedset.iter().position(|item| item.position == current_node.position) {
                    if current_node.position == *start {
                        println!("({:?},{:?}, {:?})", x, y, cost + heuristic_cost_estimate(&neighbour_position, &target));
                    }
                    let neighbour_node = Node{ parent_index: parent_index as i32, position: neighbour_position, cost_g: cost, 
                        cost_h: heuristic_cost_estimate(&neighbour_position, &target)};
                    openset.add(neighbour_node);
                }else{
                    println!("This part of the pathfinding should never happen");
                    return None;
                }
                
            }

           
        }
    }
    println!("Run out of nodes");
    None
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

fn check_collision(tilemap: &Tilemap, position: &UVec2) -> bool{
    // Skip colliding neighbours as no route
    let mut valid_tile = false;
    let mut collision = false;
    for layer in 0..TilemapLayer::EndOfLayers as u32 {
        if let Some(tile) = tilemap.get_tile_from_grid_pos(
            position, layer,
        ) {
            valid_tile = true;
            if tile.has_collision { 
                collision = true;
                break;
             }
        };
    }
    // No tile found
    collision || !valid_tile
}