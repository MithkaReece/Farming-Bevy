use bevy::prelude::*;

#[derive(Component)]
pub struct Memory {
    max_water_memory: usize,
    max_food_memory: usize,
    pub water_memory: Vec<UVec2>,
    pub food_memory: Vec<UVec2>,
}

impl Memory {
    pub fn new(max_water_memory: usize, max_food_memory: usize) -> Self{
        Memory {
            max_water_memory,
            max_food_memory,
            water_memory: Vec::new(),
            food_memory: Vec::new(),
        }
    }

    pub fn add_water(&mut self, position: UVec2){
        if self.water_memory.contains(&position) {
            return;
        }

        if self.water_memory.len() >= self.max_water_memory {
            self.water_memory.remove(0);
        }

        self.water_memory.push(position);
    }

    pub fn add_food(&mut self, position: UVec2){
        if self.food_memory.contains(&position) {
            return;
        }

        if self.food_memory.len() >= self.max_food_memory {
            self.food_memory.remove(0);
        }

        self.food_memory.push(position);
    }

    pub fn remove_water(&mut self, position: UVec2){
        if let Some(index) = self.water_memory.iter().position(|found_pos| found_pos == &position) {
            self.water_memory.remove(index);
        }
    }

    pub fn remove_food(&mut self, position: UVec2){
        if let Some(index) = self.food_memory.iter().position(|found_pos| found_pos == &position) {
            self.food_memory.remove(index);
        }
    }

    pub fn top_water(&self) -> Option<&UVec2> {
        self.water_memory.first()
    }

    pub fn top_food(&self) -> Option<&UVec2> {
        self.food_memory.first()
    }
}
