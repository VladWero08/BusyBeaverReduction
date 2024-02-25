use std::sync::mpsc::Sender;
use std::thread;

use crate::delta::transition_function::TransitionFunction;
use crate::turing_machine::direction::Direction;
use crate::turing_machine::special_states::SpecialStates;

pub struct FilterCompile {}

impl FilterCompile {
    /// Creates a new thread were all the `TransitionFunction` from the `Vec`
    /// will be filtered.
    ///
    /// Returns the filtered `Vec`.
    pub fn filter(
        mut transition_functions: Vec<TransitionFunction>,
        tx: Sender<Vec<TransitionFunction>>,
    ) {
        // create a new thread, move the transition functions into it
        // and filter them all
        thread::spawn(move || {
            transition_functions
                .retain(|transition_function| Self::filter_all(transition_function) == true);

            // send the filtered transition functions
            // through the channel
            tx.send(transition_functions).unwrap();
        });
    }

    /// Applies all filters of the `FilterCompile` struct to the provided
    /// `TransitionFunction` and returns true if they were `all` passed.
    fn filter_all(transition_function: &TransitionFunction) -> bool {
        return Self::filter_start_state_moves_left(transition_function)
            && Self::filter_start_state_moves_right_loop(transition_function)
            && Self::filter_moves_to_halting_state(transition_function)
            && Self::filter_moves_right_loop(transition_function)
            && Self::filter_no_moves_to_halting_state(transition_function)
            && Self::filter_no_symbol_writing(transition_function);
    }

    /// Checks whether the start state of the transition function
    /// provided will try to move to the LEFT on input 0.
    fn filter_start_state_moves_left(transition_function: &TransitionFunction) -> bool {
        let start_state_key: &(u8, u8) = &(SpecialStates::STATE_START.value(), 0);
        let start_state_value: Option<&(u8, u8, Direction)> =
            transition_function.transitions.get(start_state_key);

        match start_state_value {
            Some(transition) => {
                return !(transition.2 == Direction::LEFT);
            }
            None => {
                return true;
            }
        }
    }

    /// Checks whether the start state of the transition function
    /// provided will run into a self loop, moving infinitely to the right
    /// and writing 0s on the tape (self loops).
    fn filter_start_state_moves_right_loop(transition_function: &TransitionFunction) -> bool {
        let start_state_key: &(u8, u8) = &(SpecialStates::STATE_START.value(), 0);
        let start_state_value: Option<&(u8, u8, Direction)> =
            transition_function.transitions.get(start_state_key);

        match start_state_value {
            Some(transition) => {
                return !(transition.0 == SpecialStates::STATE_START.value()
                    && transition.2 == Direction::RIGHT);
            }
            None => {
                return true;
            }
        }
    }

    /// Checks whether the start state of the transition function
    /// will move to the right and enter a state that will be self looping.
    fn filter_moves_right_loop(transition_function: &TransitionFunction) -> bool {
        let start_state_key: &(u8, u8) = &(SpecialStates::STATE_START.value(), 0);
        let start_state_value: Option<&(u8, u8, Direction)> =
            transition_function.transitions.get(start_state_key);
    
        let next_state_key: (u8, u8);

        // update the following state's key only if the key for 
        // the starting state exists
        match start_state_value {
            Some(transition) => {
                next_state_key = (transition.0, transition.1);
            }
            None => {
                return true;
            }
        }

        let next_state_value: Option<&(u8, u8, Direction)> = 
            transition_function.transitions.get(&next_state_key);

        // check if the following state will self loop
        match next_state_value {
            Some(transition) => {
                return !(transition.0 == next_state_key.0
                    && transition.2 == Direction::RIGHT);
            } 
            None => {
                return true;
            }
        }
    }

    /// Checks whether the start state of the transition function
    /// will move directly to the halting state.
    fn filter_moves_to_halting_state(transition_function: &TransitionFunction) -> bool {
        let start_state_key: &(u8, u8) = &(SpecialStates::STATE_START.value(), 0);
        let start_state_value: Option<&(u8, u8, Direction)> =
            transition_function.transitions.get(start_state_key);

        match start_state_value {
            Some(transition) => {
                return !(transition.0 == SpecialStates::STATE_HALT.value());
            }
            None => {
                return true;
            }
        }
    }

    /// Check if there is at least one transition that will
    /// go to the halting state.
    fn filter_no_moves_to_halting_state(transition_function: &TransitionFunction) -> bool {
        // iterate and check for every transition if it goes
        // to the halting state, and if at least one does, the filter is passed
        for transition in transition_function.transitions.clone() {
            let transition_next = transition.1;
            let transition_next_state = transition_next.0;

            if transition_next_state == SpecialStates::STATE_HALT.value() {
                return true;
            }
        }

        return false;
    }

    /// Check if there is at least one transition that will
    /// write a `1` symbol on the tape.
    fn filter_no_symbol_writing(transition_function: &TransitionFunction) -> bool {
        for transition in transition_function.transitions.clone() {
            let transition_next = transition.1;
            let transition_next_symbol = transition_next.1;

            if transition_next_symbol == 1 {
                return true;
            }
        }
    
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::delta::transition::Transition;

    #[test]
    fn test_filter_start_state_moves_right_loop() {
        let mut transition_function: TransitionFunction = TransitionFunction::new();

        transition_function.add_transition(Transition {
            from_state: SpecialStates::STATE_START.value(),
            from_symbol: 0,
            to_state: 0,
            to_symbol: 0,
            direction: Direction::RIGHT,
        });

        assert_eq!(
            FilterCompile::filter_start_state_moves_right_loop(&transition_function),
            false
        );
    }

    #[test]
    fn test_filter_left_move_start_state() {
        let mut transition_function: TransitionFunction = TransitionFunction::new();

        transition_function.add_transition(Transition {
            from_state: SpecialStates::STATE_START.value(),
            from_symbol: 0,
            to_state: 0,
            to_symbol: 0,
            direction: Direction::LEFT,
        });

        assert_eq!(
            FilterCompile::filter_start_state_moves_left(&transition_function),
            false
        );
    }
}
