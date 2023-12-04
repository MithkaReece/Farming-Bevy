use bevy::prelude::*;

use super::Tilemap;

#[derive(Component)]
pub struct Target {
    pub path: Vec<UVec2>,
}

impl Target {
    pub fn get_target(&self) -> Option<&UVec2> {
        self.path.last()
    }

    pub fn has_target(&mut self, current_pos: &Vec2) -> bool {
        let current_target = match self.path.last() {
            Some(next_position) => next_position,
            None => return false,
        };

        // Check if target has been reached
        let min_distance = 1.0;
        // Close enough to current target
        if current_target.as_vec2().distance(*current_pos) >= min_distance {
            return true;
        };

        // Go to next target on pathfind
        return if let Some(_) = self.path.pop() {
            self.path.last() != None
        } else {
            false
        };
    }

    pub fn set_random_target(
        &mut self,
        tilemap: &Tilemap,
        full_scaling_factor: f32,
        current_pos: &Vec2,
    ) {
        let bounds = BoxBounds {
            min_x: 0.0,
            max_x: (tilemap.dimensions.x * tilemap.chunk_size as u32) as f32 * full_scaling_factor,
            min_y: 0.0,
            max_y: (tilemap.dimensions.y * tilemap.chunk_size as u32) as f32 * full_scaling_factor,
        };

        self.path.clear();
        // Make new target
        self.path.push(self.random_location_in_box(
            current_pos,
            10.0 * full_scaling_factor,
            &bounds,
        ));
    }

    fn random_location_in_box(
        &self,
        origin: &Vec2,
        max_distance: f32,
        bounds: &BoxBounds,
    ) -> UVec2 {
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
