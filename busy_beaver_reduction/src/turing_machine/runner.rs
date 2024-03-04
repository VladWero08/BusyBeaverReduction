use crate::delta::transition_function::TransitionFunction;
use crate::turing_machine::turing_machine::TuringMachine;
use threadpool::ThreadPool;

const WORKERS: usize = 8;

pub struct TuringMachineRunner {
    pub pool: ThreadPool,
}

impl TuringMachineRunner {
    pub fn new() -> Self {
        TuringMachineRunner {
            pool: ThreadPool::new(WORKERS),
        }
    }

    /// Given an array of `TransitionFunction`s, use the pool of threads
    /// to create a new Turing Machine for each one
    /// and start executing them.
    pub fn run(&mut self, transition_functions: Vec<TransitionFunction>) {
        for transition_function in transition_functions {
            self.pool.execute(move || {
                let mut turing_machine = TuringMachine::new(transition_function);
                turing_machine.execute();
            })
        }
    }
}
