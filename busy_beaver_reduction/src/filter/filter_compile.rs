use std::thread;

use crate::delta::transition::Transition;
use crate::delta::transition_function::TransitionFunction;
use crate::turing_machine::direction::Direction;
use crate::turing_machine::special_states::SpecialStates;

pub struct FilterCompile {}

impl FilterCompile {
    /// Creates a new thread were all the `TransitionFunction` from the `Vec`
    /// will be filtered.
    ///
    /// Returns the filtered `Vec`.
    pub fn filter(mut transition_functions: Vec<TransitionFunction>) -> Vec<TransitionFunction> {
        // create a new thread, move the transition functions into it
        // and filter them all
        let filter_thread = thread::spawn(move || {
            transition_functions
                .retain(|transition_function| Self::filter_all(transition_function) == true);

            return transition_functions;
        });

        // wait for the thread to finish and
        // get the filtered transition functions
        transition_functions = filter_thread
            .join()
            .expect("Thread panicked while filtering the transition function!");

        return transition_functions;
    }

    /// Applies all filters of the `FilterCompile` struct to the provided
    /// `TransitionFunction` and returns true if they were `all` passed.
    fn filter_all(transition_function: &TransitionFunction) -> bool {
        return Self::filter_left_move_start_state(transition_function)
            && Self::filter_right_move_loop(transition_function);
    }

    /// Checks whether the start state of the transition function
    /// provided will try to move to the LEFT on input 0.
    fn filter_left_move_start_state(transition_function: &TransitionFunction) -> bool {
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
    /// and writing 0s on the tape.
    fn filter_right_move_loop(transition_function: &TransitionFunction) -> bool {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_right_move_loop() {
        let mut transition_function: TransitionFunction = TransitionFunction::new();

        transition_function.add_transition(Transition {
            from_state: SpecialStates::STATE_START.value(),
            from_symbol: 0,
            to_state: 0,
            to_symbol: 0,
            direction: Direction::RIGHT,
        });

        assert_eq!(
            FilterCompile::filter_right_move_loop(&transition_function),
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
            FilterCompile::filter_left_move_start_state(&transition_function),
            false
        );
    }

    #[test]
    fn test_filter() {
        let mut transition_function_01: TransitionFunction = TransitionFunction::new();
        let mut transition_function_02: TransitionFunction = TransitionFunction::new();

        transition_function_01.add_transition(Transition {
            from_state: SpecialStates::STATE_START.value(),
            from_symbol: 0,
            to_state: 0,
            to_symbol: 0,
            direction: Direction::RIGHT,
        });

        transition_function_02.add_transition(Transition {
            from_state: SpecialStates::STATE_START.value(),
            from_symbol: 0,
            to_state: 0,
            to_symbol: 0,
            direction: Direction::LEFT,
        });

        let mut transition_functions: Vec<TransitionFunction> = Vec::new();

        transition_functions.push(transition_function_01);
        transition_functions.push(transition_function_02);

        assert_eq!(FilterCompile::filter(transition_functions), Vec::new());
    }
}
