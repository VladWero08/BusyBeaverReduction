mod database;
mod delta;
mod filter;
mod generator;
mod logger;
mod turing_machine;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::delta::transition_function::TransitionFunction;
use crate::filter::filter::Filter;
use crate::generator::generator::Generator;
use crate::logger::logger::load_logger;

#[tokio::main]
async fn main() {
    load_logger();

    let (tx_unfiltered, rx_unfiltered): (
        Sender<Vec<TransitionFunction>>,
        Receiver<Vec<TransitionFunction>>,
    ) = channel();

    let (tx_filtered, rx_filtered): (
        Sender<Vec<TransitionFunction>>,
        Receiver<Vec<TransitionFunction>>,
    ) = channel();

    let mut generator_: Generator = Generator::new(2, tx_unfiltered, rx_filtered);

    let filter_handle = thread::spawn(move || {
        let mut filter_: Filter = Filter::new(tx_filtered, rx_unfiltered);

        filter_.receive_all_unfiltered();
    });

    let generator_handle = thread::spawn(move || {
        generator_.generate();
    });

    let _ = filter_handle.join();
    let _ = generator_handle.join();
}
