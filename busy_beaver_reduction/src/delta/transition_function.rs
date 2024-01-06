use std::collections::HashMap;
use crate::delta::transition::Transition;

pub struct TransitionFunction {
    pub transitions: HashMap<(u8, u8), (u8, u8, u8)>,
}

impl TransitionFunction {
    pub fn new() -> Self {
        TransitionFunction {
            transitions: HashMap::new()
        }
    }

    /// Given a `Transition`, inserts it into the HashMap,
    /// indexing it by (`from_state`, `from_symbol`).
    /// 
    /// It means that the transition will be identified
    /// by the current state of the Turing Machine and 
    /// the current symbol where the `head` is pointing at.
    pub fn add_transition(&mut self, transition: Transition) { 
        self.transitions.insert(
            (transition.from_state, transition.from_symbol), 
            (transition.to_state, transition.to_symbol, transition.direction)
        );
    }
}