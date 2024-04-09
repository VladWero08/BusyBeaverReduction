use crate::delta::transition_function::TransitionFunction;
use crate::turing_machine::direction::Direction;
use crate::turing_machine::special_states::SpecialStates;

/// Implements filter techniques for `TransitionFunction`s that
/// have been `partially generated`.
///
/// This filtering is used during the generation of all
/// transition functions, to reduce the number of functions
/// that need to be generated.
pub struct FilterGenerate {}

impl FilterGenerate {
    /// Applies all filters of the `FilterGenerate` struct to the provided
    /// `TransitionFunction` and returns true if they were `all` passed.
    pub fn filter_all(transition_function: &TransitionFunction) -> bool {
        return Self::filter_start_state_moves_into_loop(transition_function)
            && Self::filter_moves_into_loop(transition_function)
            && Self::filter_moves_to_halting_state(transition_function);
        // && Self::filter_start_state_moves_left(transition_function);
    }

    /// Checks whether the start state of the transition function
    /// provided will try to move to the LEFT on input 0.
    fn filter_start_state_moves_left(transition_function: &TransitionFunction) -> bool {
        let start_state_key: &(u8, u8) = &(SpecialStates::StateStart.value(), 0);
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
    /// provided will run into a self loop, moving infinitely to 
    /// the right / left and writing 0s on the tape (self loops).
    fn filter_start_state_moves_into_loop(transition_function: &TransitionFunction) -> bool {
        let start_state_key: &(u8, u8) = &(SpecialStates::StateStart.value(), 0);
        let start_state_value: Option<&(u8, u8, Direction)> =
            transition_function.transitions.get(start_state_key);

        match start_state_value {
            Some(transition) => {
                return !(transition.0 == SpecialStates::StateStart.value());
            }
            None => {
                return true;
            }
        }
    }

    /// Checks whether the start state of the transition function
    /// will move directly to the halting state.
    fn filter_moves_to_halting_state(transition_function: &TransitionFunction) -> bool {
        let start_state_key: &(u8, u8) = &(SpecialStates::StateStart.value(), 0);
        let start_state_value: Option<&(u8, u8, Direction)> =
            transition_function.transitions.get(start_state_key);

        match start_state_value {
            Some(transition) => {
                return !(transition.0 == SpecialStates::StateHalt.value());
            }
            None => {
                return true;
            }
        }
    }

    /// Checks whether the start state of the transition function
    /// will move into a state that will be self looping.
    /// 
    /// In order to move into a state that will be self looping,
    /// it needs to keep its direction, as follows:
    /// 
    /// - `start_state` -- RIGHT --> `self looping state` to RIGHT
    /// - `start_state` -- LEFT --> `self looping state` to LEFT
    fn filter_moves_into_loop(transition_function: &TransitionFunction) -> bool {
        let start_state_key: &(u8, u8) = &(SpecialStates::StateStart.value(), 0);
        let start_state_value: Option<&(u8, u8, Direction)> =
            transition_function.transitions.get(start_state_key);
        // the direction in which the tape head
        // will be moving
        let start_state_direction: Direction;

        let next_state_key: (u8, u8);

        // update the following state's key only if the key for
        // the starting state exists
        match start_state_value {
            Some(transition) => {
                start_state_direction = transition.2;
                next_state_key = (transition.0, 0);
            }
            None => {
                return true;
            }
        }

        let next_state_value: Option<&(u8, u8, Direction)> =
            transition_function.transitions.get(&next_state_key);

        // check if the following state will self loop,
        // by keeping moving in the same direction and staying
        // in the same state
        match next_state_value {
            Some(transition) => {
                return !(transition.0 == next_state_key.0
                    && transition.2 == start_state_direction);
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
    use crate::delta::transition::Transition;

    #[test]
    fn filter_start_state_moves_right_loop() {
        let mut transition_function: TransitionFunction = TransitionFunction::new(0, 0);

        transition_function.add_transition(Transition {
            from_state: SpecialStates::StateStart.value(),
            from_symbol: 0,
            to_state: 0,
            to_symbol: 0,
            direction: Direction::RIGHT,
        });

        assert_eq!(
            FilterGenerate::filter_start_state_moves_into_loop(&transition_function),
            false
        );
    }

    #[test]
    fn filter_left_move_start_state() {
        let mut transition_function: TransitionFunction = TransitionFunction::new(0, 0);

        transition_function.add_transition(Transition {
            from_state: SpecialStates::StateStart.value(),
            from_symbol: 0,
            to_state: 0,
            to_symbol: 0,
            direction: Direction::LEFT,
        });

        assert_eq!(
            FilterGenerate::filter_start_state_moves_left(&transition_function),
            false
        );
    }

    #[test]
    fn filter_moves_to_halting_state() {
        let mut transition_function: TransitionFunction = TransitionFunction::new(2, 2);

        transition_function.add_transition(Transition {
            from_state: SpecialStates::StateStart.value(),
            from_symbol: 0,
            to_state: SpecialStates::StateHalt.value(),
            to_symbol: 1,
            direction: Direction::RIGHT,
        });

        transition_function.add_transition(Transition {
            from_state: SpecialStates::StateStart.value(),
            from_symbol: 1,
            to_state: 1,
            to_symbol: 0,
            direction: Direction::RIGHT,
        });

        let filter_result = FilterGenerate::filter_moves_to_halting_state(&transition_function);
        assert_eq!(filter_result, false);
    }

    #[test]
    fn filter_moves_right_loop() {
        let mut transition_function: TransitionFunction = TransitionFunction::new(2, 2);

        transition_function.add_transition(Transition {
            from_state: SpecialStates::StateStart.value(),
            from_symbol: 0,
            to_state: 1,
            to_symbol: 1,
            direction: Direction::RIGHT,
        });

        transition_function.add_transition(Transition {
            from_state: 1,
            from_symbol: 0,
            to_state: 1,
            to_symbol: 0,
            direction: Direction::RIGHT,
        });

        let filter_result = FilterGenerate::filter_moves_into_loop(&transition_function);
        assert_eq!(filter_result, false);
    }
}
