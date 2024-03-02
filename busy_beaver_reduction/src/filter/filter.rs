use std::sync::mpsc::{Receiver, Sender};

use crate::delta::transition_function::TransitionFunction;
use crate::filter::filter_compile::FilterCompile;

pub struct Filter {
    pub number_of_batches: usize,

    pub tx_filtered_functions: Option<Sender<Vec<TransitionFunction>>>,
    pub rx_unfiltered_functions: Receiver<Vec<TransitionFunction>>,
}

impl Filter {
    pub fn new(
        number_of_batches: usize,
        tx_filtered_functions: Sender<Vec<TransitionFunction>>,
        rx_unfiltered_functions: Receiver<Vec<TransitionFunction>>,
    ) -> Self {
        Filter {
            number_of_batches: number_of_batches,
            tx_filtered_functions: Some(tx_filtered_functions),
            rx_unfiltered_functions: rx_unfiltered_functions,
        }
    }

    /// Listens to the chanel where the `Generator` will publish
    /// transition functions, than proceeds to filter them
    /// and return them back to the generator through another channel.
    pub fn receive_all_unfiltered(&mut self) {
        for transition_functions in self.rx_unfiltered_functions.iter() {
            self.send_filtered(transition_functions);
        }

        let _ = std::mem::replace(&mut self.tx_filtered_functions, None);
    }

    /// Filters the received transition functions and
    /// send them back to the `Generator` that produced them.
    fn send_filtered(&self, transition_functions: Vec<TransitionFunction>) {
        match &self.tx_filtered_functions {
            Some(sender) => {
                let tx_filtered_functions_clone = sender.clone();
                // filter the received tranisition functions
                FilterCompile::filter(transition_functions, tx_filtered_functions_clone);
            }
            None => {}
        }
    }
}
