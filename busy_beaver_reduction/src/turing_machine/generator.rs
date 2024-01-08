use crate::delta::transition::{self, Transition};
use crate::delta::transition_function::{TransitionFunction, self};
use crate::turing_machine::direction::Direction;
use crate::turing_machine::turing_machine::TuringMachine;

const directions: [Direction; 2] = [Direction::LEFT, Direction::RIGHT];

pub struct Generator {
    pub states: Vec<u8>,
    /// considers input alphabet = tape alphabet
    pub alphabet: Vec<u8>,
    pub all_transitions: Vec<Transition>
}

impl Generator {
    /// Generates every transition that is possible
    /// withing the `states` and `alphabet` of
    pub fn generate_all_transitions(&mut self) {
        for &from_state in self.states.iter() {
            for &from_symbol in self.alphabet.iter() {
                for &to_state in self.states.iter() {
                    for &to_symbol in self.states.iter() {
                        for &direction in directions.iter() {
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
    pub fn generate_all_transition_functions(&mut self, number_of_transitions: u8) {
        // if the number of desired transitions is bigger
        // than the number of possible transitions for a transition function, exit
        let maximum_number_of_transitions: u8 = self.states.len() as u8 * self.alphabet.len() as u8;
        
        if number_of_transitions > maximum_number_of_transitions {
            return;
        }

        // if transitions were not generated, generate them
        if self.all_transitions.is_empty() {
            self.generate_all_transitions();
        }
    
        for i in 0..self.all_transitions.len() {
            for j in i+1..self.all_transitions.len() {
                for k in j+1..self.all_transitions.len() {
                    let indexes: Vec<usize> = vec![i, j, k];
                    let transition_function: TransitionFunction = self.generate_transition_function(indexes);
                }
            }
        }
     }

    /// Given a list of indexes, returns a `TransitionFunction` with
    /// the transitions (from `all_transitions`) corresponding to the indexes.
    pub fn generate_transition_function(&self, transitions: Vec<usize>) -> TransitionFunction{
        let mut transition_function: TransitionFunction = TransitionFunction::new();

        for index in transitions {
            transition_function.add_transition(self.all_transitions[index].clone());
        }

        return transition_function;
    }

}
