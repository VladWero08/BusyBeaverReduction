use crate::delta::transition::Transition;
use crate::delta::transition_function::TransitionFunction;
use crate::turing_machine::direction::Direction;

const DIRECTIONS: [Direction; 2] = [Direction::LEFT, Direction::RIGHT];

pub struct Generator {
    pub states: Vec<u8>,
    /// considers input alphabet = tape alphabet
    pub alphabet: Vec<u8>,
    pub all_transitions: Vec<Transition>,
}

impl Generator {
    /// Generates every transition that is possible
    /// withing the `states` and `alphabet` of
    pub fn generate_all_transitions(&mut self) {
        for &from_state in self.states.iter() {
            for &from_symbol in self.alphabet.iter() {
                for &to_state in self.states.iter() {
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
        // if the number of desired transitions is bigger
        // than the number of possible transitions for a transition function, exit
        let maximum_number_of_transitions: usize =
            self.states.len() as usize * self.alphabet.len() as usize;

        // if transitions were not generated, generate them
        if self.all_transitions.is_empty() {
            self.generate_all_transitions();
        }

        let mut indexes: &mut Vec<usize> = (&mut vec![]);

        self.generate_all_transition_combinations(0, indexes, 0, maximum_number_of_transitions);

    }

    /// Generates all possible combinations of the transitions.
    /// Uses recursion to compute combinations of `N` taken by `K`, where
    /// 
    /// `N` = total number of transitions with Q states an S symbols
    /// `K` = 1 ... maximum number of transitions possible to combine (`Q * S`)
    fn generate_all_transition_combinations(
        &self,
        index: usize,
        transition_indexes: &mut Vec<usize>,
        deepness: usize,
        max_deepness: usize,
    ) {
        // if the maximum depth was reached, exit
        if deepness == max_deepness {
            return;
        }

        // otherwise, start adding transitions to the current combination
        // and compute a new transition functions
        for i in index..self.all_transitions.len() {
            transition_indexes.push(i);

            self.generate_transition_function(&transition_indexes);
            // recursive call to continue on adding 
            // new transitions to the combintation
            self.generate_all_transition_combinations(
                i + 1,
                transition_indexes,
                deepness + 1,
                max_deepness,
            );

            transition_indexes.pop();
        }
    }

    /// Given a list of indexes, returns a `TransitionFunction` with
    /// the transitions (from `all_transitions`) corresponding to the indexes.
    fn generate_transition_function(
        &self,
        transitions: &Vec<usize>,
    ) -> TransitionFunction {
        let mut transition_function: TransitionFunction = TransitionFunction::new();

        for &index in transitions {
            transition_function.add_transition(self.all_transitions[index]);
        }

        return transition_function;
    }
}
