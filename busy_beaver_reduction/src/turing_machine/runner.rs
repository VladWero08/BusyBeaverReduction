use rayon;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{Semaphore, SemaphorePermit};
use tokio::sync::mpsc::Sender;

use crate::turing_machine;
use crate::turing_machine::turing_machine::TuringMachine;
use log::{error, info};

const MAXIMUM_THREADS: usize = 10;

pub struct TuringMachineRunner {
    pub tx_turing_machines: Option<Sender<TuringMachine>>,
}

impl TuringMachineRunner {
    pub fn new(tx_turing_machine: Sender<TuringMachine>) -> Self {
        TuringMachineRunner {
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
    /// machines in the database.
    pub async fn run(&mut self, turing_machines: Vec<TuringMachine>) {
        info!(
            "Started running turing machine. {} total machines to run...",
            turing_machines.len()
        );
        
        let pool = rayon::ThreadPoolBuilder::new().num_threads(16).build().unwrap();
        let mut finished_turing_machines: Vec<TuringMachine> = Vec::new();

        pool.install(|| {
            for mut turing_machine in turing_machines {
                turing_machine.execute();
                finished_turing_machines.push(turing_machine);
            }
        });

        for turing_machine  in finished_turing_machines {
            let turing_machine_channel: Sender<TuringMachine> = self.tx_turing_machines.clone().unwrap();
            let _ = turing_machine_channel.send(turing_machine).await;
        }

        // after the running of every TuringMachine,
        // drop the communication channel with the database
        let _ = std::mem::replace(&mut self.tx_turing_machines, None);
        info!("Dropped communication channel betwenn Turing Machine and Database Manager runners.");
    }

    pub async fn run_old(&mut self, turing_machines: Vec<TuringMachine>) {
        info!(
            "Started running turing machine. {} total machines to run...",
            turing_machines.len()
        );
 
        let semaphore = Arc::new(Semaphore::new(MAXIMUM_THREADS));
        let mut turing_machine_executions: Vec<tokio::task::JoinHandle<()>> = vec![];

        for mut turing_machine in turing_machines {
            let turing_machine_channel: Sender<TuringMachine> =
                self.tx_turing_machines.clone().unwrap();
            let semaphore = semaphore.clone();

            let turing_machine_execution = tokio::spawn(async move {
                // wait for the permission to execute the Turing machine
                let permit: SemaphorePermit = semaphore.acquire().await.unwrap();

                // build the turing machine based on the transition
                // function received, than execute it
                let (send, recv) = tokio::sync::oneshot::channel();

                // create a rayon thread to execute the CPU bound task,
                // the task of executing the turing machine
                rayon::spawn(move || {
                    turing_machine.execute();
                    let _ = send.send(turing_machine);
                });

                let _ = match recv.await {
                    // if no error occured, send the turing machine that
                    // was executed to the database manager runner, to update its entry
                    // in the database
                    Ok(turing_machine) => {
                        let _ = turing_machine_channel.send(turing_machine).await;
                    }
                    // otherwise, log the error
                    Err(e) => {
                        error!("While receving turing machine from rayon runtime {}", e);
                    }
                };

                drop(permit);
            });

            turing_machine_executions.push(turing_machine_execution);
        }

        for turing_machine_execution in turing_machine_executions {
            let _ = turing_machine_execution.await.unwrap();
        }

        info!("Finished running all the turing machines.");

        // after the running of every TuringMachine,
        // drop the communication channel with the database
        let _ = std::mem::replace(&mut self.tx_turing_machines, None);
        info!("Dropped communication channel betwenn Turing Machine and Database Manager runners.");
    }
}
