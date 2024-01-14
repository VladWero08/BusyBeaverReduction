use std::sync::mpsc::{Receiver, Sender};

use crate::delta::transition_function::TransitionFunction;
use crate::filter::filter_compile::FilterCompile;

pub struct Filter {
    tx_filtered_functions: Sender<Vec<TransitionFunction>>,
    rx_unfiltered_functions: Receiver<Vec<TransitionFunction>>,
    unfiltered_functions: i128,
}

impl Filter {
    /// Filters the received transition functions and
    /// send them back to the `Generator` that produced them.
    fn send_filtered(&self, mut transition_functions: Vec<TransitionFunction>) {
        // filter the received tranisition functions
        transition_functions = FilterCompile::filter(transition_functions);
        // send the transition functions through the channel
        self.tx_filtered_functions
            .send(transition_functions)
            .unwrap();
    }

    /// Listens for every batch of `TransitionFunction`s that will
    /// be received from the generator.
    pub fn receive_all_unfiltered(&self) {
        for _ in 0..self.unfiltered_functions {
            self.receive_unfiltered();
        }
    }

    /// Listens to the chanel where the `Generator` will publish
    /// transition functions, which will be `filtered` and returned
    /// to the generator.
    fn receive_unfiltered(&self) {
        let transition_functions: Vec<TransitionFunction> =
            self.rx_unfiltered_functions.recv().unwrap();

        self.send_filtered(transition_functions);
    }
}
