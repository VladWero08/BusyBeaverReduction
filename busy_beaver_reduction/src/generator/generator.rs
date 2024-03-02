use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use log::info;

use crate::delta::transition_function::TransitionFunction;
use crate::generator::generator_transition_function::GeneratorTransitionFunction;

const BATCH_SIZE: usize = 100;

pub struct Generator {
    pub number_of_states: u8,
    pub transition_functions: Vec<TransitionFunction>,
    pub number_of_batches: usize,

    pub tx_unfiltered_functions: Option<Sender<Vec<TransitionFunction>>>,
    pub rx_filtered_functions: Receiver<Vec<TransitionFunction>>,
}

impl Generator {
    pub fn new(
        number_of_states: u8,
        tx_unfiltered_functions: Sender<Vec<TransitionFunction>>,
        rx_filtered_functions: Receiver<Vec<TransitionFunction>>,
    ) -> Self {
        let maximum_no_of_transition_functions: usize =
            GeneratorTransitionFunction::get_maximum_no_of_transition_functions(number_of_states);
        let mut number_of_batches: usize = maximum_no_of_transition_functions / BATCH_SIZE;

        // because the number of maximum transition functions
        // might not be divisible by the batch size, add + 1
        // for the rest of transition functions left out
        if maximum_no_of_transition_functions % BATCH_SIZE != 0 {
            number_of_batches += 1;
        }

        Generator {
            transition_functions: Vec::new(),
            number_of_states: number_of_states,
            number_of_batches: number_of_batches,
            tx_unfiltered_functions: Some(tx_unfiltered_functions),
            rx_filtered_functions: rx_filtered_functions,
        }
    }

    /// Creates a new thread were the all the generation
    /// of transition functions will take place.
    fn send_unfiletered(&mut self) {
        let mut generator: GeneratorTransitionFunction =
            GeneratorTransitionFunction::new(self.number_of_states);

        match &self.tx_unfiltered_functions {
            Some(sender) => {
                let tx_unfiltered_functions: Sender<Vec<TransitionFunction>> = sender.clone();

                thread::spawn(move || {
                    generator
                        .generate_all_transition_functions(tx_unfiltered_functions, BATCH_SIZE);
                });
            }
            None => {}
        }

        let _ = std::mem::replace(&mut self.tx_unfiltered_functions, None);
    }

    /// Listens for filtered transitions functions, and once received
    /// extend the `self.transition_functions` vector.
    fn receive_filtered(&mut self) {
        for transition_functions_filtered in self.rx_filtered_functions.iter() {
            self.transition_functions
                .extend(transition_functions_filtered);
        }

        self.filter_status();
    }

    /// Calculates what percentage of the transition functions
    /// have been filtered by the compile time filter.
    fn filter_status(&mut self) {
        let maximum_no_of_transition_functions: usize =
            GeneratorTransitionFunction::get_maximum_no_of_transition_functions(
                self.number_of_states,
            );

        let filtered_total = maximum_no_of_transition_functions - self.transition_functions.len();
        let filtered_percentage = filtered_total * 100 / maximum_no_of_transition_functions;

        info!(
            "Filtered {}% of the transition functions. ({} / {})",
            filtered_percentage, filtered_total, maximum_no_of_transition_functions
        );
    }

    pub fn generate(&mut self) {
        self.send_unfiletered();
        self.receive_filtered();
    }
}
