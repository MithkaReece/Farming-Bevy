use std::{
    collections::{HashMap, VecDeque},
    thread::current,
};

#[derive(Clone)]
pub struct BT<A, C> {
    root: BehaviourTreeNode<A, C>,
    pub blackboard: Blackboard<C>,
}

impl<A, C: Clone + std::fmt::Debug + Eq + PartialEq> BT<A, C> {
    pub fn new(bt: BehaviourTreeNode<A, C>) -> Self {
        //TODO: Traverse bt to fill in conditions in blackboard
        println!("Start traversal");
        let mut conditions = VecDeque::new();
        bt.retrieve_conditions(&mut conditions);
        println!("{:?}", conditions);

        let blackboard = Blackboard::new(conditions);

        Self {
            root: bt,
            blackboard,
        }
    }
    // Root execute to rootnode
    pub fn execute<F>(&mut self, mut action_mapper: &mut F) -> Status
    where
        F: FnMut(&mut A) -> Status,
    {
        self.root.execute(&mut action_mapper, &mut self.blackboard)
    }
}

pub enum Status {
    Success,
    Failure,
    Running,
}

#[derive(Clone)]
pub enum BehaviourTreeNode<A, C> {
    Sequence {
        children: Vec<BehaviourTreeNode<A, C>>,
        current_child_index: usize,
    },
    Selector {
        children: Vec<BehaviourTreeNode<A, C>>,
        current_child_index: usize,
    },
    Inverter(Box<BehaviourTreeNode<A, C>>),
    Action(A),
    Condition(C),
}

use Status::*;

use crate::config::blackboard::Blackboard;

use super::blackboard::ConditionRef;
impl<A, C: Clone + std::fmt::Debug + Eq + PartialEq> BehaviourTreeNode<A, C> {
    pub fn new_sequence(children: Vec<BehaviourTreeNode<A, C>>) -> Self {
        BehaviourTreeNode::Sequence {
            children,
            current_child_index: 0,
        }
    }
    pub fn new_selector(children: Vec<BehaviourTreeNode<A, C>>) -> Self {
        BehaviourTreeNode::Selector {
            children,
            current_child_index: 0,
        }
    }

    pub fn execute<F>(
        &mut self,
        mut action_mapper: &mut F,
        mut blackboard: &mut Blackboard<C>,
    ) -> Status
    where
        F: FnMut(&mut A) -> Status,
    {
        if blackboard.should_reset() {
            return self.reset_and_execute(action_mapper, blackboard)
        }

        fn inner_execute<A, C: Eq + PartialEq, F>(
            node: &mut BehaviourTreeNode<A, C>,
            action_mapper: &mut F,
            blackboard: &mut Blackboard<C>,
        ) -> Status
        where
            F: FnMut(&mut A) -> Status,
        {
            match node {
                BehaviourTreeNode::Sequence {
                    children,
                    current_child_index,
                } => {
                    while *current_child_index < children.len() {
                        let child_status = inner_execute(
                            &mut children[*current_child_index],
                            action_mapper,
                            blackboard,
                        );
                        match child_status {
                            Success => {
                                *current_child_index += 1;
                            }
                            Failure => return Failure,
                            Running => return Running,
                        }
                    }
                    *current_child_index = 0;
                    Success
                }
                BehaviourTreeNode::Selector {
                    children,
                    current_child_index,
                } => {
                    while *current_child_index < children.len() {
                        let child_status = inner_execute(
                            &mut children[*current_child_index],
                            action_mapper,
                            blackboard,
                        );
                        match child_status {
                            Success => return Success,
                            Failure => {
                                *current_child_index += 1;
                            }
                            Running => return Running,
                        }
                    }
                    *current_child_index = 0;
                    Failure
                }
                BehaviourTreeNode::Inverter(child) => {
                    match inner_execute(child, action_mapper, blackboard) {
                        Success => Failure,
                        Failure => Success,
                        Running => Running,
                    }
                }
                BehaviourTreeNode::Action(action) => (action_mapper)(action),
                BehaviourTreeNode::Condition(condition) => {
                    match blackboard.get_condition_state(condition) {
                        None => {
                            println!("Could not find condition in blackboard");
                            Failure
                        }
                        Some(state) => {
                            blackboard.increment_condition_counter();
                            if state {
                                Success
                            } else {
                                Failure
                            }
                        }
                    }
                }
            }
        }

        blackboard.reset_condition_counter();
        inner_execute(self, &mut action_mapper, &mut blackboard)
    }


    // Only difference is current_child_index is always set to 0 rather 
    // than the saved value
    pub fn reset_and_execute<F>(
        &mut self,
        mut action_mapper: &mut F,
        mut blackboard: &mut Blackboard<C>,
    ) -> Status
    where
        F: FnMut(&mut A) -> Status,
    {
        // Flip off the reset so next iteration is normal execute
        blackboard.has_reset(); 

        fn inner_execute<A, C: Eq + PartialEq, F>(
            node: &mut BehaviourTreeNode<A, C>,
            action_mapper: &mut F,
            blackboard: &mut Blackboard<C>,
        ) -> Status
        where
            F: FnMut(&mut A) -> Status,
        {
            match node {
                BehaviourTreeNode::Sequence {
                    children,
                    current_child_index,
                } => {
                    *current_child_index = 0;
                    while *current_child_index < children.len() {
                        let child_status = inner_execute(
                            &mut children[*current_child_index],
                            action_mapper,
                            blackboard,
                        );
                        match child_status {
                            Success => {
                                *current_child_index += 1;
                            }
                            Failure => return Failure,
                            Running => return Running,
                        }
                    }
                    *current_child_index = 0;
                    Success
                }
                BehaviourTreeNode::Selector {
                    children,
                    current_child_index,
                } => {
                    *current_child_index = 0;
                    while *current_child_index < children.len() {
                        let child_status = inner_execute(
                            &mut children[*current_child_index],
                            action_mapper,
                            blackboard,
                        );
                        match child_status {
                            Success => return Success,
                            Failure => {
                                *current_child_index += 1;
                            }
                            Running => return Running,
                        }
                    }
                    *current_child_index = 0;
                    Failure
                }
                BehaviourTreeNode::Inverter(child) => {
                    match inner_execute(child, action_mapper, blackboard) {
                        Success => Failure,
                        Failure => Success,
                        Running => Running,
                    }
                }
                BehaviourTreeNode::Action(action) => (action_mapper)(action),
                BehaviourTreeNode::Condition(condition) => {
                    match blackboard.get_condition_state(condition) {
                        None => {
                            println!("Could not find condition in blackboard");
                            Failure
                        }
                        Some(state) => {
                            blackboard.increment_condition_counter();
                            if state {
                                Success
                            } else {
                                Failure
                            }
                        }
                    }
                }
            }
        }

        blackboard.reset_condition_counter();
        inner_execute(self, &mut action_mapper, &mut blackboard)
    }

    pub fn retrieve_conditions(&self, mut conditions: &mut VecDeque<ConditionRef<C>>) {
        fn inner_execute<A, C: Clone + std::fmt::Debug>(
            node: &BehaviourTreeNode<A, C>,
            conditions: &mut VecDeque<ConditionRef<C>>,
        ) {
            match node {
                BehaviourTreeNode::Sequence {
                    children,
                    current_child_index: _,
                } => {
                    for child in children {
                        inner_execute(child, conditions);
                    }
                }
                BehaviourTreeNode::Selector {
                    children,
                    current_child_index: _,
                } => {
                    for child in children {
                        inner_execute(child, conditions);
                    }
                }
                BehaviourTreeNode::Inverter(child) => {
                    inner_execute(child, conditions);
                    let condition = match conditions.back_mut() {
                        None => {
                            println!("Wtf can't get last conditions (behaviour-tree)");
                            return;
                        }
                        Some(condition) => condition,
                    };
                    condition.invert();
                }
                BehaviourTreeNode::Action(_) => {}
                BehaviourTreeNode::Condition(condition) => {
                    conditions.push_back(ConditionRef::new((*condition).clone()));
                }
            }
        }

        inner_execute(self, &mut conditions)
    }
}
