use std::sync::mpsc::{Receiver, Sender};

use crate::delta::transition_function::TransitionFunction;
use crate::filter::filter_compile::FilterCompile;
use crate::turing_machine::direction::Direction;

const DIRECTIONS: [Direction; 2] = [Direction::LEFT, Direction::RIGHT];
const ALPHABET: [u8; 2] = [0, 1];

pub struct Filter {
    pub tx_filtered_functions: Option<Sender<Vec<TransitionFunction>>>,
    pub rx_unfiltered_functions: Receiver<Vec<TransitionFunction>>,
    pub filter_compile: FilterCompile,
}

impl Filter {
    pub fn new(
        tx_filtered_functions: Sender<Vec<TransitionFunction>>,
        rx_unfiltered_functions: Receiver<Vec<TransitionFunction>>,
        number_of_states: u8,
    ) -> Self {
        Filter {
            tx_filtered_functions: Some(tx_filtered_functions),
            rx_unfiltered_functions: rx_unfiltered_functions,
            filter_compile: FilterCompile::new(
                number_of_states as usize,
                ALPHABET.len(),
                DIRECTIONS.len(),
            ),
        }
    }

    /// Listens to the chanel where the `Generator` will publish
    /// transition functions, than proceeds to filter them
    /// and return them back to the generator through another channel.
    pub fn receive_all_unfiltered(&mut self) {
        for transition_functions in self.rx_unfiltered_functions.iter() {
            // filters the received transition functions and
            // send them back to the `Generator` that produced them.
            match &self.tx_filtered_functions {
                Some(sender) => {
                    let tx_filtered_functions_clone = sender.clone();
                    // filter the received tranisition functions
                    self.filter_compile
                        .filter(transition_functions, tx_filtered_functions_clone);
                }
                None => {}
            }
        }

        self.filter_compile.display_filtering_results();

        let _ = std::mem::replace(&mut self.tx_filtered_functions, None);
    }
}
