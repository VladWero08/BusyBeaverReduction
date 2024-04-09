use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use tokio;

use log::info;

use crate::database::manager::DatabaseManager;
use crate::database::runner::DatabaseManagerRunner;
use crate::delta::transition_function::TransitionFunction;
use crate::filter::filter::Filter;
use crate::generator::generator::Generator;
use crate::turing_machine::runner::TuringMachineRunner;
use crate::turing_machine::turing_machine::TuringMachine;

const BATCH_SIZE: usize = 100;

pub struct Mediator {
    number_of_states: u8,
    turing_machines: Vec<TuringMachine>,
}

impl Mediator {
    pub fn new(number_of_states: u8) -> Self {
        Mediator {
            number_of_states: number_of_states,
            turing_machines: vec![],
        }
    }

    /// Checks if the generation already took place, aka
    /// there are turing machines with the desired number of states
    /// in the database. If there aren'y any, it:
    ///
    /// - creates a new thread in which the `Filter`
    /// will be listening for unfiltered transition functions and
    /// will send them filtered back to the `Generator`.
    ///
    /// - creates a new thread in which the `Generator`
    /// will be generating unfiltered transition functions and
    /// will wait to receive the filtered from the `Filter`.
    pub async fn generate_and_filter(&mut self) {
        // try loading turing machines from the database
        if self.load_turing_machines().await == true {
            info!(
                "Loaded turing machines from the database, a total of {}.",
                self.turing_machines.len()
            );
            return;
        }

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
            let mut filter = Filter::new(tx_filtered_functions, rx_unfiltered_functions);

            filter.receive_all_unfiltered();
        });

        // create a copy of number of states
        let number_of_states = self.number_of_states;
        // creates a new thread for the generator
        let generator_handle = thread::spawn(move || {
            let mut generator = Generator::new(
                number_of_states,
                tx_unfiltered_functions,
                rx_filtered_functions,
            );

            generator.generate();

            // returns the transition functions generated
            // by the generator
            return generator.transition_functions;
        });

        // waits for both threads to finish running
        let _ = filter_handle.join();
        let transition_functions_generated = generator_handle.join().unwrap();

        self.make_turing_machines(transition_functions_generated);
        self.insert_turing_machines().await;
    }

    /// After the generator and filter finished to create
    /// the first instances of transition functions, use them
    /// to create instances of `TuringMachine`s.
    fn make_turing_machines(&mut self, transition_functions: Vec<TransitionFunction>) {
        for transition_function in transition_functions {
            let turing_machine = TuringMachine::new(transition_function);
            self.turing_machines.push(turing_machine);
        }
    }

    /// Tries to retrieve any turing machine from the database
    /// that has `number_of_states` states.
    ///
    /// If any exist, set the turing machines of the
    /// mediator to be equal to the ones extracted from
    /// the database.
    ///
    /// Used when trying to generate turing machines, in order
    /// to skip some computations.
    async fn load_turing_machines(&mut self) -> bool {
        let db_option = DatabaseManager::new().await;

        match db_option {
            // if the database manager was succesfully created,
            // try to select all the turing machines with the
            // desired number of states
            Some(mut database_manager) => {
                let tm_option = database_manager
                    .select_turing_machines_to_run(self.number_of_states, 2)
                    .await;

                match tm_option {
                    // if the select did not fail, check if
                    // any such Turing Machines exist in the database
                    Some(turing_machines) => {
                        // if they do, it means the generation was already done,
                        // so save the turing machines directly
                        if turing_machines.len() > 0 {
                            self.turing_machines = turing_machines;
                        }

                        return self.turing_machines.len() > 0;
                    }
                    None => {}
                }
            }
            None => {}
        }

        return false;
    }

    /// After the Turing Machines were made from the
    /// generated Transition Functions, this function inserts
    /// them in the database in batches.
    async fn insert_turing_machines(&self) {
        let db_option = DatabaseManager::new().await;

        match db_option {
            Some(mut database_manager) => {
                // iterate through the turing machines and
                // insert them in batches
                for batch in (0..self.turing_machines.len()).step_by(BATCH_SIZE) {
                    let mut batch_size = BATCH_SIZE;

                    if self.turing_machines.len() - batch < BATCH_SIZE {
                        batch_size = self.turing_machines.len() - batch;
                    }

                    database_manager
                        .batch_insert_turing_machines(
                            &self.turing_machines[batch..batch + batch_size],
                        )
                        .await;

                    // log after each 10 batch insertion
                    if batch % 1000 == 0 {
                        info!("Inserted {}th batch of 100 Turing Machines...", batch / 100);
                    }
                }

                info!("Inserted all Turing Machines in the database!");
            }
            None => {}
        }
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
            tokio::sync::mpsc::Sender<TuringMachine>,
            tokio::sync::mpsc::Receiver<TuringMachine>,
        ) = tokio::sync::mpsc::channel(100);

        // creates a new thread for the database insertions
        let database_handler = tokio::spawn(async {
            let mut database_manager_runner = DatabaseManagerRunner::new(rx_turing_machine);
            database_manager_runner.receive_turing_machines().await;
        });

        // creates a new thread to run turing machines
        let tm_runner_handler = tokio::spawn(async {
            let mut tm_runner = TuringMachineRunner::new(tx_turing_machine);
            tm_runner.run(self.turing_machines).await;
        });

        // wait for both threads to finish
        let _ = database_handler.await;
        let _ = tm_runner_handler.await;
    }
}
