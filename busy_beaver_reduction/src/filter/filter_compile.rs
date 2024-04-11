use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::thread;

use regex::Regex;

use crate::delta::transition_function::TransitionFunction;
use crate::delta::{transition, transition_function};
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

            transition_functions = Self::filter_existing_templates(transition_functions);

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

            if transition_next_state == SpecialStates::StateHalt.value() {
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

    /// Filters out Transition Functions that behave in the same way
    /// with another Transition Function that already exists in the
    /// `templates` vector.
    ///
    /// Two Transition Functions behave in the same way in the case when
    /// by interchanging some states of one of them, we get the other
    /// Transition Function.
    ///
    /// ### Example
    /// f: (2, 1) -> (3, 1, R)
    /// f: (3, 0) -> (2, 1, L)
    ///
    /// g: (3, 1) -> (2, 1, R)
    /// g: (2, 0) -> (3, 1, L)
    ///
    /// If we interchange appearences of states `2` and `3` for transition
    /// function g, we get f.
    fn filter_existing_templates(
        mut transition_functions: Vec<TransitionFunction>,
    ) -> Vec<TransitionFunction> {
        let mut turing_machines_templates: Vec<Vec<(Regex, u8, u8)>> = Vec::new();
        let mut transition_functions_to_remove: Vec<usize> = Vec::new();

        for index in 0..transition_functions.len() {
            let filter = FilterCompile::filter_against_templates(
                &transition_functions[index],
                &turing_machines_templates,
            );

            // if the filter was passed, it means it is a new configuration
            // of transition function, add it to the templates
            if filter == true {
                let new_template = FilterCompile::retrieve_template(&transition_functions[index]);
                turing_machines_templates.push(new_template);
            }
            // otheriwse, keep the index in a vector
            // in order to delete this transition function
            // after filtering all of them
            else {
                transition_functions_to_remove.push(index);
            }
        }

        for index in transition_functions_to_remove {
            transition_functions.remove(index);
        }

        return transition_functions;
    }

    /// Check whether a transition function already has
    /// an equivalent template which behaves in the same way
    fn filter_against_templates(
        transition_function: &TransitionFunction,
        turing_machines_templates: &Vec<Vec<(Regex, u8, u8)>>,
    ) -> bool {
        for template in turing_machines_templates {
            let mut template_matched: bool = true;
            let mut transition_function_encoded = transition_function.encode();
            // holds the mapping of the state of the template
            // to the states of the current transition,
            // if at any point this mapping is broken, it means it does
            // not respect the current template
            let mut states_mapping: HashMap<u8, u8> = HashMap::new();

            for transition_regex in template {
                // if the current regex does not match the encoding,
                // this template cannot be matched
                if !transition_regex.0.is_match(&transition_function_encoded) {
                    template_matched = false;
                    break;
                }

                // extract the states from the transition
                let Some(states) = transition_regex.0.captures(&transition_function_encoded) else {
                    continue;
                };
                let from_state = states[1].as_bytes()[0];
                let to_state = states[2].as_bytes()[0];

                // check if the states from the template exist in the
                // states mapping; if they do, check if they are in correlance
                // with the mapping
                // check for from state
                if states_mapping.contains_key(&transition_regex.1) {
                    let state_mapped = states_mapping.get(&transition_regex.1).unwrap();

                    if *state_mapped != from_state {
                        template_matched = false;
                        break;
                    }
                } else {
                    states_mapping.insert(transition_regex.1, from_state);
                }

                // check for to state
                if states_mapping.contains_key(&transition_regex.2) {
                    let state_mapped = states_mapping.get(&transition_regex.2).unwrap();

                    if *state_mapped != to_state {
                        template_matched = false;
                        break;
                    }
                } else {
                    states_mapping.insert(transition_regex.2, to_state);
                }

                // after using the regex for extracting information
                // about a transition from the transition function, delete
                // the transition from the encoding to prevent it from being
                // picked up again by an identical regex
                transition_function_encoded = transition_regex
                    .0
                    .replace_all(transition_function_encoded.as_str(), "")
                    .into_owned();
            }

            // if the template matched, it means it did not
            // pass the filter, return false
            if template_matched == true {
                return false;
            }
        }

        return true;
    }

    /// Retrieve a regex for each transition in a transition function,
    /// that will extract `from state` and `to state` from another
    /// transition function that is possible to behave in the same way.
    fn retrieve_template(transition_function: &TransitionFunction) -> Vec<(Regex, u8, u8)> {
        let mut template: Vec<(Regex, u8, u8)> = Vec::new();

        for (key, value) in &transition_function.transitions {
            let transition_regex = Regex::new(
                format!(r"(\d),{},(\d),{},{}", key.1, value.1, value.2.value()).as_str(),
            )
            .unwrap();
            // add the pair (regex, from state, to state) into the list
            template.push((transition_regex, key.0, value.0));
        }

        return template;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{delta::transition::Transition, turing_machine::direction::Direction};

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

    #[test]
    fn filter_against_templates() {
        let mut transition_function_01: TransitionFunction = TransitionFunction::new(3, 3);
        let mut transition_function_02: TransitionFunction = TransitionFunction::new(3, 3);
        let mut transition_function_03: TransitionFunction = TransitionFunction::new(3, 3);
        let mut transition_function_04: TransitionFunction = TransitionFunction::new(3, 3);

        // initiate transition function 1
        transition_function_01.add_transition(Transition::new_params(1, 1, 2, 1, Direction::RIGHT));
        transition_function_01.add_transition(Transition::new_params(1, 0, 0, 1, Direction::LEFT));
        transition_function_01.add_transition(Transition::new_params(2, 1, 1, 1, Direction::LEFT));
        transition_function_01.add_transition(Transition::new_params(2, 0, 2, 0, Direction::RIGHT));

        // initiate transition function 2
        transition_function_02.add_transition(Transition::new_params(2, 1, 1, 1, Direction::RIGHT));
        transition_function_02.add_transition(Transition::new_params(2, 0, 0, 1, Direction::LEFT));
        transition_function_02.add_transition(Transition::new_params(1, 1, 2, 1, Direction::LEFT));
        transition_function_02.add_transition(Transition::new_params(1, 0, 1, 0, Direction::RIGHT));

        // initiate transition function 3
        transition_function_03.add_transition(Transition::new_params(2, 1, 1, 1, Direction::RIGHT));
        transition_function_03.add_transition(Transition::new_params(2, 0, 0, 1, Direction::LEFT));
        transition_function_03.add_transition(Transition::new_params(1, 1, 2, 1, Direction::LEFT));
        transition_function_03.add_transition(Transition::new_params(1, 0, 1, 0, Direction::LEFT));

        // initiate transition function 4
        transition_function_04.add_transition(Transition::new_params(2, 1, 1, 1, Direction::RIGHT));
        transition_function_04.add_transition(Transition::new_params(2, 0, 0, 0, Direction::LEFT));
        transition_function_04.add_transition(Transition::new_params(1, 1, 2, 1, Direction::LEFT));
        transition_function_04.add_transition(Transition::new_params(1, 0, 1, 0, Direction::RIGHT));

        let transition_functions: Vec<TransitionFunction> = vec![
            transition_function_01.clone(),
            transition_function_02.clone(),
            transition_function_03.clone(),
            transition_function_04.clone(),
        ];
        let transition_functions_filtered =
            FilterCompile::filter_existing_templates(transition_functions);

        assert_eq!(
            transition_functions_filtered.contains(&transition_function_01),
            true
        );
        assert_eq!(
            transition_functions_filtered.contains(&transition_function_02),
            false
        );
        assert_eq!(
            transition_functions_filtered.contains(&transition_function_03),
            true
        );
        assert_eq!(
            transition_functions_filtered.contains(&transition_function_04),
            true
        );
    }
}
