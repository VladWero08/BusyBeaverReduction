mod delta;
mod filter;
mod turing_machine;

use delta::transition_function::TransitionFunction;
use std::sync::mpsc::{self, channel};

use crate::turing_machine::generator::Generator;
use std::time::Instant;

fn main() {
    let mut generator: Generator = Generator::n_state_generator(2);

    let now = Instant::now();
    generator.generate_all_transition_functions();
    let elapsed = now.elapsed();

    println!(
        "Total number of functions generated {} in {:.2?}",
        generator.n, elapsed
    );
}
