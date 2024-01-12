mod delta;
mod turing_machine;

use crate::delta::transition::Transition;
use crate::turing_machine::generator::Generator;

fn main() {
    let mut generator: Generator = Generator::n_state_generator(2);

    generator.generate_all_transition_functions();

    println!("Total number of functions generated {}", generator.n);
}
