use std::collections::VecDeque;
use std::sync::mpsc::Sender;

use log::info;

use crate::delta::transition::Transition;
use crate::delta::transition_function::{self, TransitionFunction};
use crate::filter::filter_generate::FilterGenerate;
use crate::turing_machine::direction::Direction;
use crate::turing_machine::special_states::SpecialStates;

const DIRECTIONS: [Direction; 2] = [Direction::LEFT, Direction::RIGHT];
const ALPHABET: [u8; 2] = [0, 1];
const GENERATION_ALGORITHM: &str = "DEQUE";

pub struct GeneratorTransitionFunction {
    pub states: Vec<u8>,
    pub states_final: Vec<u8>,
    pub all_transitions: Vec<Transition>,
    pub filter_generate: FilterGenerate,
}

impl GeneratorTransitionFunction {
    pub fn new(number_of_states: u8) -> Self {
        // initiate the states vector with the starting state
        let mut states: Vec<u8> = vec![SpecialStates::StateStart.value()];
        let mut states_final: Vec<u8> = vec![SpecialStates::StateStart.value()];

        // for the rest of the states, assign each one
        // a number from 1 to n
        for state_number in 1..number_of_states {
            states.push(state_number);
            states_final.push(state_number);
        }

        // fot the states_final vector also add the halting state
        states_final.push(SpecialStates::StateHalt.value());

        info!(
            "Generator, based on backtracking, with {} states has been created!",
            number_of_states
        );

        return GeneratorTransitionFunction {
            states: states,
            states_final: states_final,
            all_transitions: vec![],
            filter_generate: FilterGenerate::new(
                number_of_states as usize,
                ALPHABET.len(),
                DIRECTIONS.len(),
            ),
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

    /// Given a `Vec<usize>` that contains indexes of the transitions from `self.all_transitions`
    /// used for making a transition function, build the `TransitionFunction` object and filter it
    /// using the `GenerateFilter`.
    pub fn generate_filter_by_vec(&mut self, indexes: &Vec<u8>) -> bool {
        let mut transition_function =
            TransitionFunction::new(self.states.len() as u8, ALPHABET.len() as u8);

        for index in indexes {
            transition_function.add_transition(self.all_transitions[*index as usize]);
        }

        self.filter_generate.filter_all(&transition_function)
    }

    /// Generates every transition that is possible
    /// withing the `states` and `alphabet` of
    pub fn generate_all_transitions(&mut self) {
        let alphabet = ALPHABET
            .iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<_>>()
            .join(", ");

        let total_possible_transitions = self.states.len()
            * ALPHABET.len()
            * self.states_final.len()
            * ALPHABET.len()
            * DIRECTIONS.len();

        info!(
            "Generating all transitions with {} states, on alphabet [{}].",
            self.states.len(),
            alphabet
        );

        for &from_state in self.states.iter() {
            for &from_symbol in ALPHABET.iter() {
                for &to_state in self.states_final.iter() {
                    // it is necessary to only generate
                    // one transition that goes into the halting state,
                    // only to take into account when writing a 1

                    // this is a built in filter for generation,
                    // that will create less transition functions
                    if to_state == SpecialStates::StateHalt.value() {
                        let transition = Transition {
                            from_state: from_state,
                            from_symbol: from_symbol,
                            to_state: to_state,
                            to_symbol: 1,
                            direction: Direction::RIGHT,
                        };

                        self.all_transitions.push(transition);
                    } else {
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
        }

        info!(
            "Generated a total of {} transitions.",
            self.all_transitions.len()
        );

        info!(
            "Only {} out of {} possible transitions are useful.",
            self.all_transitions.len(),
            total_possible_transitions,
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

        // if transitions were not generated, generate them
        if self.all_transitions.is_empty() {
            self.generate_all_transitions();
        }

        info!("Generating all possible transition functions.");

        match GENERATION_ALGORITHM {
            "DEQUE" => {
                // generate all possible functions by combining
                // every possible function using a deque with TransitionFunctions
                self.generate_all_transition_combiation_dequeue(
                    maximum_number_of_transitions,
                    &tx_unfiltered_functions,
                    batch_size,
                );
            }
            "DEQUE_VEC" => {
                // generate all possible functions by combining
                // every possible function using a deque with Vec<u8> transition indexes
                self.generate_all_transition_combiation_dequeue_with_vec(
                    maximum_number_of_transitions as u8,
                    &tx_unfiltered_functions,
                    batch_size,
                );
            }
            _ => {
                // where all transition functions will be computed
                let transition_function: &mut TransitionFunction =
                    &mut TransitionFunction::new(self.states.len() as u8, ALPHABET.len() as u8);
                let transition_functions_set: &mut Vec<TransitionFunction> = &mut Vec::new();
                let index: usize = 0;
                let deepness: usize = 0;

                self.generate_all_transition_combinations(
                    index,
                    transition_function,
                    transition_functions_set,
                    &tx_unfiltered_functions.clone(),
                    deepness,
                    maximum_number_of_transition_functions,
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
            }
        }

        info!(
            "Generated a total of {} transition functions.",
            maximum_number_of_transition_functions
        );

        self.filter_generate.display_filtering_results();
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
                if self.filter_generate.filter_all(transition_function) == true {
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

    /// Generates all possible combinations of transition
    /// with a queue, instead of making use of recursion.
    ///
    /// This method allows better control of the order in
    /// which the transition functions will be generated.
    pub fn generate_all_transition_combiation_dequeue(
        &mut self,
        maximum_number_of_transitions: usize,
        tx_unfiltered_functions: &Sender<Vec<TransitionFunction>>,
        batch_size: usize,
    ) {
        let mut transition_functions_set: Vec<TransitionFunction> = Vec::new();
        let maximum_possibilites_for_entry =
            self.states.len() * ALPHABET.len() * DIRECTIONS.len() + 1;
        let mut queue: VecDeque<TransitionFunction> = VecDeque::new();

        // initialise the queue with transition function that separately
        // contain all the transitions of the form (0, 0) ->
        for index in 0..maximum_possibilites_for_entry {
            let mut transition_function: TransitionFunction =
                TransitionFunction::new(self.states.len() as u8, ALPHABET.len() as u8);
            transition_function.add_transition(self.all_transitions[index]);

            if self.filter_generate.filter_all(&transition_function) == true {
                queue.push_back(transition_function);
            }
        }

        let mut deepness = 1;

        while queue.len() != 0 {
            // extract the oldest transition function in the queue
            let mut transition_function = queue.pop_front().unwrap();
            let transition_function_length = transition_function.transitions.len();

            if transition_function_length > deepness {
                info!("Reached deepnes {}", transition_function_length);
                info!("Generation queue size: {}", queue.len());
                deepness += 1;
            }

            // if the transition function reached the desired number of transitions,
            // add it to the set of transition functions;
            if transition_function_length == maximum_number_of_transitions {
                transition_functions_set.push(transition_function);

                // if the transition function set reached the batch size,
                // send the unfiltered transitions to the filter
                if transition_functions_set.len() == batch_size {
                    tx_unfiltered_functions
                        .send(transition_functions_set)
                        .unwrap();
                    transition_functions_set = Vec::new();
                }
            } else {
                // because the transition were generated sequentally, the first ones
                // target (q_{0}, 0), than (q_{0}, 1), and so on... iterate through the
                // next transition that need to be added and check their validty
                for index in maximum_possibilites_for_entry * transition_function_length
                    ..maximum_possibilites_for_entry * (transition_function_length + 1)
                {
                    let transition_key: &(u8, u8) = &(
                        self.all_transitions[index].from_state,
                        self.all_transitions[index].from_symbol,
                    );

                    if !transition_function.transitions.contains_key(transition_key) {
                        transition_function.add_transition(self.all_transitions[index]);

                        // check if the transition function passes the
                        // generation filters
                        if self.filter_generate.filter_all(&transition_function) == true {
                            queue.push_back(transition_function.clone());
                        }

                        transition_function.transitions.remove(transition_key);
                    }
                }
            }

            if queue.len() < queue.capacity() / 2 {
                queue.shrink_to_fit();
            }
        }
    }

    /// Generates all possible combinations of transition
    /// with a dequeue, instead of making use of recursion.
    ///
    /// This method allows better control of the order in
    /// which the transition functions will be generated.
    ///
    /// In addition to the original method of generation with deque,
    /// instead of keeping all `TransitionFunction`s in the deque,
    /// only the indexes of `self.all_transitions` for a transition function
    /// are kept in a `Vec`, which is added in deque.
    ///
    /// To filter the `Vec` of transition indexes, a `TransitionFunction`
    /// object is built before the filtering is done.
    pub fn generate_all_transition_combiation_dequeue_with_vec(
        &mut self,
        maximum_number_of_transitions: u8,
        tx_unfiltered_functions: &Sender<Vec<TransitionFunction>>,
        batch_size: usize,
    ) {
        let mut transition_functions_set: Vec<TransitionFunction> = Vec::new();
        let maximum_possibilites_for_entry =
            (self.states.len() * ALPHABET.len() * DIRECTIONS.len() + 1) as u8;
        let mut queue: VecDeque<Vec<u8>> = VecDeque::new();

        // initialise the queue with transition function that separately
        // contain all the transitions of the form (0, 0) ->
        for index in 0u8..maximum_possibilites_for_entry {
            let transitions_indexes: Vec<u8> = Vec::from([index]);

            if self.generate_filter_by_vec(&transitions_indexes) == true {
                queue.push_back(transitions_indexes);
            }
        }

        let mut deepness = 1;

        while queue.len() != 0 {
            // extract the oldest transition function in the queue
            let mut transitions_vec = queue.pop_front().unwrap();
            let transitions_vec_length = transitions_vec.len() as u8;

            if transitions_vec_length > deepness {
                info!("Reached deepnes {}", transitions_vec_length);
                info!("Generation queue size: {}", queue.len());
                deepness += 1;
            }

            // because the transition were generated sequentally, the first ones
            // target (q_{0}, 0), than (q_{0}, 1), and so on... iterate through the
            // next transition that need to be added and check their validty
            for index in maximum_possibilites_for_entry * transitions_vec_length
                ..maximum_possibilites_for_entry * (transitions_vec_length + 1)
            {
                transitions_vec.push(index);

                // check if the transition function passes the
                // generation filters
                if self.generate_filter_by_vec(&transitions_vec) == true {
                    if transitions_vec_length + 1 == maximum_number_of_transitions {
                        let mut transition_function =
                            TransitionFunction::new(self.states.len() as u8, ALPHABET.len() as u8);

                        for index in transitions_vec.clone() {
                            transition_function
                                .add_transition(self.all_transitions[index as usize]);
                        }

                        transition_functions_set.push(transition_function);

                        // if the transition function set reached the batch size,
                        // send the unfiltered transitions to the filter
                        if transition_functions_set.len() == batch_size {
                            tx_unfiltered_functions
                                .send(transition_functions_set)
                                .unwrap();
                            transition_functions_set = Vec::new();
                        }
                    } else {
                        queue.push_back(transitions_vec.clone());
                    }
                }

                transitions_vec.pop();
            }

            if queue.len() < queue.capacity() / 2 {
                queue.shrink_to_fit();
            }
        }
    }
}
