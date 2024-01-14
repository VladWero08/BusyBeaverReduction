use crate::delta::transition::Transition;
use crate::delta::transition_function::TransitionFunction;
use crate::turing_machine::direction::Direction;
use crate::turing_machine::special_states::SpecialStates;

const DIRECTIONS: [Direction; 2] = [Direction::LEFT, Direction::RIGHT];

pub struct Generator {
    pub states: Vec<u8>,
    pub states_final: Vec<u8>,
    /// considers input alphabet = tape alphabet
    pub alphabet: Vec<u8>,
    pub all_transitions: Vec<Transition>,
    pub n: i128,
}

impl Generator {
    pub fn n_state_generator(n: u8) -> Self {
        // initiate the states vector with the starting state
        let mut states: Vec<u8> = vec![SpecialStates::STATE_START.value()];
        let mut states_final: Vec<u8> = vec![SpecialStates::STATE_START.value()];

        // for the rest of the states, assign each one
        // a number from 1 to n
        for state_number in 1..n {
            states.push(state_number);
            states_final.push(state_number);
        }

        // fot the states_final vector also add the halting state
        states_final.push(SpecialStates::STATE_HALT.value());

        return Generator {
            states: states,
            states_final: states_final,
            alphabet: vec![0, 1],
            all_transitions: vec![],
            n: 0,
        };
    }

    /// Generates every transition that is possible
    /// withing the `states` and `alphabet` of
    pub fn generate_all_transitions(&mut self) {
        for &from_state in self.states.iter() {
            for &from_symbol in self.alphabet.iter() {
                for &to_state in self.states_final.iter() {
                    for &to_symbol in self.alphabet.iter() {
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

    /// Generates all the transition functions that contain exactly
    /// `number_of_transitions` transitions; N taken by K functions in total.
    ///
    ///  N = number of possible transitions
    ///  K = number of desired transitions
    ///
    pub fn generate_all_transition_functions(&mut self) {
        // desired number of transition for a transition function
        let maximum_number_of_transitions: usize =
            self.states.len() as usize * self.alphabet.len() as usize;

        // where all transition functions will be computed
        let transition_function: &mut TransitionFunction = &mut TransitionFunction::new();
        let index: usize = 0;
        let deepness: usize = 0;

        // if transitions were not generated, generate them
        if self.all_transitions.is_empty() {
            self.generate_all_transitions();
        }

        // generate all possible functions by combining
        // every possible function
        self.generate_all_transition_combinations(
            index,
            transition_function,
            deepness,
            maximum_number_of_transitions,
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
        deepness: usize,
        max_deepness: usize,
    ) {
        // if the maximum depth was reached, exit
        if deepness == max_deepness {
            self.n += 1;
            // println!("{}", transition_function.encode());
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

                // recursive call to continue on adding
                // new transitions to the combintation
                self.generate_all_transition_combinations(
                    i + 1,
                    transition_function,
                    deepness + 1,
                    max_deepness,
                );

                // after returing from the recursive call,
                // delete the transition and continue on with the others
                transition_function.transitions.remove(transition_key);
            }
        }
    }
}
