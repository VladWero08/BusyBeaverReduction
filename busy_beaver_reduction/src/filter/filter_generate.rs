use crate::delta::transition_function::TransitionFunction;
use crate::turing_machine::direction::Direction;
use crate::turing_machine::special_states::SpecialStates;
use log::info;

/// Implements filter techniques for `TransitionFunction`s that
/// have been `partially generated`.
///
/// This filtering is used during the generation of all
/// transition functions, to reduce the number of functions
/// that need to be generated.
pub struct FilterGenerate {
    halting_skippers: i64,
    start_state_loopers: i64,
    neighbour_state_loopers: i64,
    naive_beavers: i64,
    turing_machines_size: i64,
    maximum_entries: usize,
    maximum_possibilies_for_entry: usize,
}

impl FilterGenerate {
    pub fn new(number_of_states: usize, alphabet_size: usize, directions_size: usize) -> Self {
        let maximum_entries = number_of_states * alphabet_size;

        // the original number of possibilites for Q' x Gamma x Directions
        let original_maximum_possibilites_for_entry =
            alphabet_size * directions_size * (number_of_states + 1);

        // represents the possibilities of Q' x Gamma x Directions,
        // in the current representation being reduced by the halting skippers
        let maximum_possibilies_for_entry = number_of_states * alphabet_size * directions_size + 1;

        let original_turing_machines_size =
            (original_maximum_possibilites_for_entry).pow(maximum_entries as u32);
        let filtered_turing_machines_size =
            (maximum_possibilies_for_entry).pow(maximum_entries as u32);

        // compute how many Turing machines were filtered using
        // the halting skippers filter technique
        let halting_skippers = original_turing_machines_size - filtered_turing_machines_size;

        return FilterGenerate {
            halting_skippers: halting_skippers as i64,
            start_state_loopers: 0,
            neighbour_state_loopers: 0,
            naive_beavers: 0,
            turing_machines_size: original_turing_machines_size as i64,
            maximum_entries,
            maximum_possibilies_for_entry,
        };
    }

    /// Given a transition function, calculates how many
    /// transition functions were filtered by stopping generating
    /// from its state onward.
    ///
    /// The computation is done based on the number of
    /// entries left to complete in the transition function.
    pub fn get_transition_function_filtered(
        &self,
        transition_function: &TransitionFunction,
    ) -> i64 {
        let entries_left_to_complete = self.maximum_entries - transition_function.transitions.len();
        let transition_functions_filtered = self
            .maximum_possibilies_for_entry
            .pow(entries_left_to_complete as u32);

        return transition_functions_filtered as i64;
    }

    /// Applies all filters of the `FilterGenerate` struct to the provided
    /// `TransitionFunction` and returns true if they were `all` passed.
    pub fn filter_all(&mut self, transition_function: &TransitionFunction) -> bool {
        if Self::filter_start_state_moves_into_loop(transition_function) == false {
            self.start_state_loopers += self.get_transition_function_filtered(transition_function);
            return false;
        }

        if Self::filter_moves_into_neighbour_loop(transition_function) == false {
            self.neighbour_state_loopers +=
                self.get_transition_function_filtered(transition_function);
            return false;
        }

        if Self::filter_moves_to_halting_state(transition_function) == false {
            self.naive_beavers += self.get_transition_function_filtered(transition_function);
            return false;
        }

        return true;
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
    fn filter_moves_into_neighbour_loop(transition_function: &TransitionFunction) -> bool {
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

    /// Display the number of Turing machines that was filtered
    /// by each individual filter.
    pub fn display_filtering_results(&self) {
        let halting_skippers_percentage =
            self.halting_skippers as f64 * 100.0 / self.turing_machines_size as f64;
        let start_state_loopers_percentage =
            self.start_state_loopers as f64 * 100.0 / self.turing_machines_size as f64;
        let neighbour_state_loopers_percentage =
            self.neighbour_state_loopers as f64 * 100.0 / self.turing_machines_size as f64;
        let naive_beavers_percentage =
            self.naive_beavers as f64 * 100.0 / self.turing_machines_size as f64;

        let total = halting_skippers_percentage
            + start_state_loopers_percentage
            + neighbour_state_loopers_percentage
            + naive_beavers_percentage;

        info!(
            "Filtered a total of halting skippers: {:.2}%",
            self.halting_skippers as f64 * 100.0 / self.turing_machines_size as f64
        );

        info!(
            "Filtered a total of start state loopers: {:.2}%",
            self.start_state_loopers as f64 * 100.0 / self.turing_machines_size as f64
        );

        info!(
            "Filtered a total of neighbour state loopers: {:.2}%",
            self.neighbour_state_loopers as f64 * 100.0 / self.turing_machines_size as f64
        );

        info!(
            "Filtered a total of naive beavers: {:.2}%",
            self.naive_beavers as f64 * 100.0 / self.turing_machines_size as f64
        );

        info!(
            "Filtered a total of {:.2}% Turing machines with generation filters.",
            total
        );
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

        let filter_result = FilterGenerate::filter_moves_into_neighbour_loop(&transition_function);
        assert_eq!(filter_result, false);
    }
}
