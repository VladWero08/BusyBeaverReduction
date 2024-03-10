use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use log::info;

use crate::database::runner::DatabaseManagerRunner;
use crate::delta::transition_function::TransitionFunction;
use crate::filter::filter::Filter;
use crate::generator::generator::Generator;
use crate::turing_machine::runner::TuringMachineRunner;
use crate::turing_machine::turing_machine::TuringMachine;

pub struct Mediator {
    number_of_states: u8,
    transition_functions: Vec<TransitionFunction>
}

impl Mediator {
    pub fn new(number_of_states: u8) -> Self {
        Mediator {
            number_of_states: number_of_states,
            transition_functions: vec![]
        }
    }

    /// Creates a new thread in which the `Filter`
    /// will be listening for unfiltered transition functions and
    /// will send them filtered back to the `Generator`.
    /// 
    /// Creates a new thread in which the `Generator`
    /// will be generating unfiltered transition functions and
    /// will wait to receive the filtered from the `Filter`.
    pub fn generate_and_filter(mut self) {
        // mpsc channel used for sending unfiltered transition functions
        // from the generator to the filter
        let (tx_unfiltered_functions, rx_unfiltered_functions): (
            Sender<Vec<TransitionFunction>>,
            Receiver<Vec<TransitionFunction>>,
        ) = channel();

        // mpsc channel used for sending filtered transition function
        // from the filter to the generator
        let (tx_filtered_functions, rx_filtered_functions): (
            Sender<Vec<TransitionFunction>>,
            Receiver<Vec<TransitionFunction>>,
        ) = channel();

        // creates a new thread for the filter
        let filter_handle = thread::spawn(move || {
            let mut filter = Filter::new(
                tx_filtered_functions,
                rx_unfiltered_functions
            );

            filter.receive_all_unfiltered();
        });

        // creates a new thread for the generator
        let generator_handle = thread::spawn(move || {
            let mut generator = Generator::new(
                self.number_of_states,
                tx_unfiltered_functions,
                rx_filtered_functions
            );

            generator.generate();

            return generator.transition_functions;
        });

        // waits for both threads to finish running
        let _ = filter_handle.join();
        let transition_functions_generated = generator_handle.join().unwrap();

        self.transition_functions = transition_functions_generated;
    }

    /// Creates a new thread that will build `TuringMachine`s based 
    /// on the transition functions generated & filtered. 
    /// Afterwards, it will execute them all and send them to the `DatabaseManagerRunner`.
    /// 
    /// Creates a new thread that will wait for executed `TuringMachine`s;
    /// after receiving them, it will insert them in the database.
    pub async fn run_and_insert(self) {
        // mpsc channel used for sending terminated turing machines
        // from the turing machine runner to the database
        let (tx_turing_machine, rx_turing_machine): (
            Sender<TuringMachine>,
            Receiver<TuringMachine>,
        ) = channel();

        // creates a new thread for the database insertions
        let database_handler = thread::spawn(move || {
            let mut database_manager_runner = DatabaseManagerRunner::new(rx_turing_machine);
            database_manager_runner.receive_turing_machines();
        });

        // creates a new thread to run turing machines
        let tm_runner_handler = thread::spawn(move || {
            let mut tm_runner = TuringMachineRunner::new(tx_turing_machine);
            tm_runner.run(self.transition_functions);
        });

        // wait for both threads to finish
        let _ = database_handler.join();
        let _ = tm_runner_handler.join();
    }
}
