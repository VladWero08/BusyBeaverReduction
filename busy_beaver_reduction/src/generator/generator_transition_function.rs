use std::sync::mpsc::Sender;

use log::info;

use crate::delta::transition::Transition;
use crate::delta::transition_function::TransitionFunction;
use crate::filter::filter_generate::FilterGenerate;
use crate::turing_machine::direction::Direction;
use crate::turing_machine::special_states::SpecialStates;

const DIRECTIONS: [Direction; 2] = [Direction::LEFT, Direction::RIGHT];
const ALPHABET: [u8; 2] = [0, 1];

pub struct GeneratorTransitionFunction {
    pub states: Vec<u8>,
    pub states_final: Vec<u8>,
    pub all_transitions: Vec<Transition>,
}

impl GeneratorTransitionFunction {
    pub fn new(number_of_states: u8) -> Self {
        // initiate the states vector with the starting state
        let mut states: Vec<u8> = vec![SpecialStates::STATE_START.value()];
        let mut states_final: Vec<u8> = vec![SpecialStates::STATE_START.value()];

        // for the rest of the states, assign each one
        // a number from 1 to n
        for state_number in 1..number_of_states {
            states.push(state_number);
            states_final.push(state_number);
        }

        // fot the states_final vector also add the halting state
        states_final.push(SpecialStates::STATE_HALT.value());

        info!(
            "Generator, based on backtracking, with {} states has been created!",
            number_of_states
        );

        return GeneratorTransitionFunction {
            states: states,
            states_final: states_final,
            all_transitions: vec![],
        };
    }

    /// Considering the following variables:
    ///
    /// - N = states alphabet size
    /// - A = tape alphabet size (0, 1) = 2
    /// - D = directions size (LEFT & RIGHT) = 2
    ///
    /// A transition function is defined as `f(N x A) = ((N + 1) x A x D)`.
    ///
    /// The number of transitions functions is `((N + 1) x A x D) ^ (N x A)`.
    pub fn get_maximum_no_of_transition_functions(number_of_states: u8) -> usize {
        let domain_size: u32 = number_of_states as u32 * ALPHABET.len() as u32;
        let codomain_size: usize =
            (number_of_states + 1) as usize * ALPHABET.len() as usize * DIRECTIONS.len() as usize;

        return usize::pow(codomain_size, domain_size);
    }

    /// Generates every transition that is possible
    /// withing the `states` and `alphabet` of
    pub fn generate_all_transitions(&mut self) {
        let alphabet = ALPHABET
            .iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<_>>()
            .join(", ");

        info!(
            "Generating all transitions with {} states, on alphabet [{}].",
            self.states.len(),
            alphabet
        );

        for &from_state in self.states.iter() {
            for &from_symbol in ALPHABET.iter() {
                for &to_state in self.states_final.iter() {
                    for &to_symbol in ALPHABET.iter() {
                        for &direction in DIRECTIONS.iter() {
                            let transition: Transition = Transition {
                                from_state: from_state,
                                from_symbol: from_symbol,
                                to_state: to_state,
                                to_symbol: to_symbol,
                                direction: direction,
                            };

                            self.all_transitions.push(transition);
                        }
                    }
                }
            }
        }

        info!(
            "Generated a total of {} transitions.",
            self.all_transitions.len()
        );
    }

    /// Generates all the transition functions that contain exactly
    /// `number_of_transitions` transitions; N taken by K functions in total.
    ///
    ///  N = number of possible transitions
    ///  K = number of desired transitions
    ///
    pub fn generate_all_transition_functions(
        &mut self,
        tx_unfiltered_functions: Sender<Vec<TransitionFunction>>,
        batch_size: usize,
    ) {
        // desired number of transition for a transition function
        let maximum_number_of_transitions: usize =
            self.states.len() as usize * ALPHABET.len() as usize;
        let maximum_number_of_transition_functions: usize =
            GeneratorTransitionFunction::get_maximum_no_of_transition_functions(
                self.states.len() as u8
            );

        // where all transition functions will be computed
        let transition_function: &mut TransitionFunction = &mut TransitionFunction::new();
        let transition_functions_set: &mut Vec<TransitionFunction> = &mut Vec::new();
        let index: usize = 0;
        let deepness: usize = 0;

        // if transitions were not generated, generate them
        if self.all_transitions.is_empty() {
            self.generate_all_transitions();
        }

        info!("Generating all possible transition functions.");

        // generate all possible functions by combining
        // every possible function
        self.generate_all_transition_combinations(
            index,
            transition_function,
            transition_functions_set,
            &tx_unfiltered_functions,
            deepness,
            maximum_number_of_transitions,
            batch_size,
        );

        // if the maximum number of transition combinations
        // will not be dividable by the batch size, also send
        // the last batch if it is not empty
        if transition_functions_set.len() != 0 {
            tx_unfiltered_functions
                .send(transition_functions_set.clone())
                .unwrap();
        }

        info!(
            "Generated a total of {} transition functions.",
            maximum_number_of_transition_functions
        );
    }

    /// Generates all possible combinations of the transitions.
    /// Uses recursion to compute combinations of `N` taken by `K`, where
    ///
    /// `N` = total number of transitions with Q states an S symbols
    /// `K` = 1 ... maximum number of transitions possible to combine (`Q * S`)
    fn generate_all_transition_combinations(
        &mut self,
        index: usize,
        transition_function: &mut TransitionFunction,
        transition_functions_set: &mut Vec<TransitionFunction>,
        tx_unfiltered_functions: &Sender<Vec<TransitionFunction>>,
        deepness: usize,
        max_deepness: usize,
        batch_size: usize,
    ) {
        // if the maximum depth was reached, exit
        if deepness == max_deepness {
            // add the transition function to the set
            transition_functions_set.push(transition_function.clone());

            // check if the set reached the batch size
            if transition_functions_set.len() == batch_size {
                // send the unfiltered transitions to the filter
                tx_unfiltered_functions
                    .send(transition_functions_set.clone())
                    .unwrap();
                // empty the transition functions vector
                transition_functions_set.clear();
            }

            return;
        }

        // otherwise, start adding transitions to the current combination
        // and compute a new transition functions
        for i in index..self.all_transitions.len() {
            let transition_key: &(u8, u8) = &(
                self.all_transitions[i].from_state,
                self.all_transitions[i].from_symbol,
            );

            // if the transition functions does not contain
            // the current transition key, add the transition to
            // the transition function
            if !transition_function.transitions.contains_key(transition_key) {
                transition_function.add_transition(self.all_transitions[i]);

                // check if the transition function passes the
                // generation filters
                if FilterGenerate::filter_all(transition_function) == true {
                    // recursive call to continue on adding
                    // new transitions to the combintation
                    self.generate_all_transition_combinations(
                        i + 1,
                        transition_function,
                        transition_functions_set,
                        tx_unfiltered_functions,
                        deepness + 1,
                        max_deepness,
                        batch_size,
                    );
                }

                // after returing from the recursive call,
                // delete the transition and continue on with the others
                transition_function.transitions.remove(transition_key);
            }
        }
    }
}
