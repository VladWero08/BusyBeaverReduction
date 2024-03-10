use std::sync::mpsc::Sender;
use threadpool::ThreadPool;

use log::info;

use crate::delta::transition_function::TransitionFunction;
use crate::turing_machine::turing_machine::TuringMachine;

const WORKERS: usize = 8;
const BATCH_SIZE: usize = 100;

pub struct TuringMachineRunner {
    pub pool: ThreadPool,
    pub tx_turing_machines: Option<Sender<TuringMachine>>,
}

impl TuringMachineRunner {
    pub fn new(tx_turing_machine: Sender<TuringMachine>) -> Self {
        TuringMachineRunner {
            pool: ThreadPool::new(WORKERS),
            tx_turing_machines: Some(tx_turing_machine),
        }
    }

    /// Given an array of `TransitionFunction`s, use the pool of threads
    /// to create a new Turing Machine for each one
    /// and start executing them.
    ///
    /// After the execution, each thread from the pool will send
    /// the `TuringMachine` instance through the mpsc channel configured
    /// upon the creation of the `TuringMachineRunner`.
    ///
    /// Consumer on the other side of the mpsc channel will insert the turing
    /// machine in the database.
    pub fn run(&mut self, transition_functions: Vec<TransitionFunction>) {
        info!(
            "Started running turing machine. {} total machines to run...",
            transition_functions.len()
        );

        for transition_function in transition_functions {
            let turing_machine_channel: Sender<TuringMachine> =
                self.tx_turing_machines.clone().unwrap();

            // build the turing machine based on the transition
            // function received, than execute it
            self.pool.execute(move || {
                let mut turing_machine = TuringMachine::new(transition_function);
                turing_machine.execute();
                let _ = turing_machine_channel.send(turing_machine);
            })
        }

        info!("Finished running all the turing machines.");

        // after the running of every TuringMachine,
        // drop the communication channel with the database
        let _ = std::mem::replace(&mut self.tx_turing_machines, None);
        info!("Dropped communication channel betwenn Turing Machine runner and database.");
    }
}
