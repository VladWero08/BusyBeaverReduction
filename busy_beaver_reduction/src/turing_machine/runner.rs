use threadpool::ThreadPool;
use crate::delta::transition_function::TransitionFunction;
use crate::turing_machine::turing_machine::TuringMachine;

const WORKERS: usize = 8;

pub struct TuringMachineRunner {
    pub pool: ThreadPool
}

impl TuringMachineRunner {
    pub fn new() -> Self {
        TuringMachineRunner {
            pool: ThreadPool::new(WORKERS)
        }
    }

    pub fn run(&mut self, transition_functions: Vec<TransitionFunction>) {
        for transition_function in transition_functions {
            self.pool.execute(move || {
                let mut turing_machine = TuringMachine::new();
                turing_machine.transition_function = transition_function;
                turing_machine.execute();
            })
        }
    }
}