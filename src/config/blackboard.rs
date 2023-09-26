use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct ConditionRef<C> {
    condition: C,
    state: bool,
    is_inverted: bool,
}

impl<C> ConditionRef<C> {
    pub fn new(condition: C) -> Self {
        Self {
            condition,
            state: false,
            is_inverted: false,
        }
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn get_state_with_inversion(&self) -> bool {
      self.state ^ self.is_inverted //XOR (so inverted flips the state)
  }

    //Flips is_inverted
    pub fn invert(&mut self) {
        self.is_inverted = !self.is_inverted;
    }
}

#[derive(Clone)]
pub struct Blackboard<C> {
    conditions: VecDeque<ConditionRef<C>>,
    condition_counter: usize,
    should_reset: bool,
}

impl<C: Eq + PartialEq> Blackboard<C> {
    pub fn new(conditions: VecDeque<ConditionRef<C>>) -> Self {
        Self {
            conditions,
            condition_counter: 0,
            should_reset: false,
        }
    }

    pub fn get_condition_state(&self, condition: &C) -> Option<bool> {
        match self
            .conditions
            .iter()
            .find(|condition_ref| &condition_ref.condition == condition)
        {
            Some(found_condition_ref) => Some(found_condition_ref.get_state()),
            None => None,
        }
    }
    // condition_counter is just a pointer to the position before the stored index
    // it is not an index itself therefore it can go up to and including length
    pub fn increment_condition_counter(&mut self) {
        if self.condition_counter < self.conditions.len() {
            self.condition_counter += 1;
        } else {
            println!("Tried to counter more conditions than exist");
        }
    }

    pub fn reset_condition_counter(&mut self) {
        self.condition_counter = 0;
    }

    pub fn update_condition(&mut self, condition: &C, new_state: bool) {
        if let Some((index, found_condition_ref)) = self
            .conditions
            .iter_mut()
            .enumerate()
            .find(|(_, condition_ref)| &condition_ref.condition == condition)
        {
          found_condition_ref.state = new_state;

          // If the state after inversions is false assuming it is in a selector
          // This means the next child needs executing
          // Therefore need to re-evaluate
          if index < self.condition_counter && !found_condition_ref.get_state_with_inversion() {
              self.should_reset = true;
          }
            
        }
    }

    pub fn has_reset(&mut self) {
        self.should_reset = false;
    }

    pub fn should_reset(&self) -> bool{
        self.should_reset
    }
}
