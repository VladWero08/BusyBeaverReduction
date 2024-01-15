use std::thread;
use std::sync::mpsc::{Receiver, Sender};

use crate::generator::generator_transition_function::GeneratorTransitionFunction;
use crate::delta::transition_function::TransitionFunction;

pub struct Generator {
    pub transition_functions: Vec<TransitionFunction>,
    pub batches: i128,

    pub tx_unfiltered_functions: Sender<Vec<TransitionFunction>>,
    pub rx_filtered_functions: Receiver<Vec<TransitionFunction>>,
}

impl Generator {
    /// Creates a new thread were the all the generation
    /// of transition functions will take place.
    pub fn send_unfiletered(&self, n: u8) {
        let generator: GeneratorTransitionFunction = GeneratorTransitionFunction::n_state_generator(n);
        let tx_unfiltered_functions: Sender<Vec<TransitionFunction>> = self.tx_unfiltered_functions.clone();
        
        thread::spawn(move || {
            generator.generate_all_transition_functions(tx_unfiltered_functions);
        });
    }

    /// Listens for filtered transitions functions, and once received
    /// extend the `self.transition_functions` vector. 
    pub fn receive_filtered(&mut self) { 
        for _ in 0..self.batches {
            let transition_functions_filtered = self.rx_filtered_functions.recv().unwrap();
            self.transition_functions.extend(transition_functions_filtered);
        }
    }
}