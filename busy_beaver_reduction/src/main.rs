mod turing_machine;
mod delta;
mod generator;
mod filter;

use std::time::Instant;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

use generator::generator_transition_function::{self, GeneratorTransitionFunction};

use crate::delta::transition_function::TransitionFunction;
use crate::filter::filter::Filter;
use crate::generator::generator::Generator;

fn main() {
    let (tx_unfiltered, rx_unfiltered): (Sender<Vec<TransitionFunction>>, Receiver<Vec<TransitionFunction>>) = channel();
    let (tx_filtered, rx_filtered): (Sender<Vec<TransitionFunction>>, Receiver<Vec<TransitionFunction>>) = channel();

    let filter_handle = thread::spawn(|| {
        let filter_: Filter = Filter {
            unfiltered_functions: 10,
            tx_filtered_functions: tx_filtered,
            rx_unfiltered_functions: rx_unfiltered,
        };

        filter_.receive_all_unfiltered();
    });

    let generator_handle = thread::spawn(|| {
        let mut generator_: Generator = Generator {
            transition_functions: Vec::new(),
            batches: 10,
            tx_unfiltered_functions: tx_unfiltered,
            rx_filtered_functions: rx_filtered,
        };

        generator_.send_unfiletered(2);
        generator_.receive_filtered();
    });
}
