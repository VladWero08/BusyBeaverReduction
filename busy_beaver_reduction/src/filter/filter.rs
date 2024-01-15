use std::sync::mpsc::{Receiver, Sender};

use crate::delta::transition_function::TransitionFunction;
use crate::filter::filter_compile::FilterCompile;

pub struct Filter {
    pub unfiltered_functions: i128,
    
    pub tx_filtered_functions: Sender<Vec<TransitionFunction>>,
    pub rx_unfiltered_functions: Receiver<Vec<TransitionFunction>>,
}

impl Filter {
    /// Listens to the chanel where the `Generator` will publish
    /// transition functions, than proceeds to filter them 
    /// and return them back to the generator through another channel.
    pub fn receive_all_unfiltered(&self) {
        for _ in 0..self.unfiltered_functions {
            let transition_functions: Vec<TransitionFunction> =
                self.rx_unfiltered_functions.recv().unwrap();

            self.send_filtered(transition_functions);
        }
    }

    /// Filters the received transition functions and
    /// send them back to the `Generator` that produced them.
    fn send_filtered(&self, mut transition_functions: Vec<TransitionFunction>) {
        let mut tx_filtered_functions_clone = self.tx_filtered_functions.clone();
        // filter the received tranisition functions
        FilterCompile::filter(transition_functions, tx_filtered_functions_clone);
    }
}
