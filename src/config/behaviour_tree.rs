use std::thread::current;

pub enum Status {
    Success,
    Failure,
    Running,
}

#[derive(Clone)]
pub enum BehaviourTreeNode<A> {
    Sequence {
        children: Vec<BehaviourTreeNode<A>>,
        current_child_index: usize,
    },
    Selector {
        children: Vec<BehaviourTreeNode<A>>,
        current_child_index: usize,
    },
    Inverter(Box<BehaviourTreeNode<A>>),
    Action(A),
}

use std::collections::HashMap;

impl<A> BehaviourTreeNode<A> {
    pub fn execute<F>(&mut self, mut action_mapper: &mut F) -> Status
    where
        F: FnMut(&mut A) -> Status,
    {
        use Status::*;

        fn inner_execute<A, F>(node: &mut BehaviourTreeNode<A>, action_mapper: &mut F) -> Status
        where
            F: FnMut(&mut A) -> Status,
        {
            match node {
                BehaviourTreeNode::Sequence {
                    children,
                    current_child_index,
                } => {
                    while *current_child_index < children.len() {
                        let child_status =
                            inner_execute(&mut children[*current_child_index], action_mapper);
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
                        let child_status =
                            inner_execute(&mut children[*current_child_index], action_mapper);
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
                BehaviourTreeNode::Inverter(child) => match inner_execute(child, action_mapper) {
                    Success => Failure,
                    Failure => Success,
                    Running => Running,
                },
                BehaviourTreeNode::Action(action) => (action_mapper)(action),
            }
        }

        inner_execute(self, &mut action_mapper)
    }
}
