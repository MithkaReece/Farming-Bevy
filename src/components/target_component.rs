use bevy::prelude::*;

use super::Tilemap;

#[derive(Component)]
pub struct Target {
    pub path: Vec<UVec2>,
    pub random_pos: UVec2,
}

impl Target {
    pub fn get_target(&self) -> &UVec2{
        match self.path.last() {
            Some(position) => position,
            None => &self.random_pos,
        }
    }

    pub fn check_target_reached(&mut self, tilemap: &Tilemap, full_scaling_factor: f32, current_pos: &Vec2) {
        // Check if target has been reached
        let min_distance = 1.0;
        if self.random_pos.as_vec2().distance(*current_pos) >= min_distance { return };

        if let Some(_) = self.path.pop() {
            if let None = self.path.last() {
                self.set_random_target(tilemap, full_scaling_factor, current_pos)
            }
        }else{
            self.set_random_target(tilemap, full_scaling_factor, current_pos)
        }
    }

    fn set_random_target(&mut self, tilemap: &Tilemap, full_scaling_factor: f32, current_pos: &Vec2){
        let bounds = BoxBounds {
            min_x: 0.0,
            max_x: (tilemap.dimensions.x * tilemap.chunk_size as u32) as f32
                * full_scaling_factor,
            min_y: 0.0,
            max_y: (tilemap.dimensions.y * tilemap.chunk_size as u32) as f32
                * full_scaling_factor,
        };

        // Make new target
        self.random_pos = self.random_location_in_box(
            current_pos,
            10.0 * full_scaling_factor,
            &bounds,
        );
    }

    fn random_location_in_box(&self, origin: &Vec2, max_distance: f32, bounds: &BoxBounds) -> UVec2 {
        loop {
            let random_x = origin.x + rand::random::<f32>() * 2.0 * max_distance - max_distance;
            let random_y = origin.y + rand::random::<f32>() * 2.0 * max_distance - max_distance;
    
            if random_x >= bounds.min_x
                && random_x <= bounds.max_x
                && random_y >= bounds.min_y
                && random_y <= bounds.max_y
            {
                return UVec2::new(random_x as u32, random_y as u32);
            }
        }
    }
}


struct BoxBounds {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}