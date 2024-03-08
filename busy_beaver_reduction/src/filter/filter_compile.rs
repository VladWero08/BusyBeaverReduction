use std::sync::mpsc::Sender;
use std::thread;

use crate::delta::transition_function::TransitionFunction;
use crate::turing_machine::special_states::SpecialStates;

/// Implements filter techniques for `TransitionFunction`s that
/// have been `fully generated`, a.k.a their domain of definition
/// is fully completed.
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
    pub fn filter_all(transition_function: &TransitionFunction) -> bool {
        return Self::filter_no_moves_to_halting_state(transition_function)
            && Self::filter_no_symbol_writing(transition_function);
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
    use crate::{delta::transition::Transition, turing_machine::direction::Direction};

    use super::*;

    #[test]
    fn filter_no_moves_to_halting_state() {
        let mut transition_function: TransitionFunction = TransitionFunction::new(2, 2);

        transition_function.add_transition(Transition {
            from_state: 0,
            from_symbol: 0,
            to_state: 1,
            to_symbol: 0,
            direction: Direction::RIGHT,
        });

        transition_function.add_transition(Transition {
            from_state: 1,
            from_symbol: 0,
            to_state: 0,
            to_symbol: 0,
            direction: Direction::LEFT,
        });

        let filter_result = FilterCompile::filter_no_moves_to_halting_state(&transition_function);
        assert_eq!(filter_result, false);
    }

    #[test]
    fn filter_no_symbol_writing() {
        let mut transition_function: TransitionFunction = TransitionFunction::new(2, 2);

        transition_function.add_transition(Transition {
            from_state: 0,
            from_symbol: 0,
            to_state: 1,
            to_symbol: 0,
            direction: Direction::RIGHT,
        });

        transition_function.add_transition(Transition {
            from_state: 1,
            from_symbol: 0,
            to_state: 0,
            to_symbol: 0,
            direction: Direction::RIGHT,
        });

        let filter_result = FilterCompile::filter_no_symbol_writing(&transition_function);
        assert_eq!(filter_result, false);
    }
}