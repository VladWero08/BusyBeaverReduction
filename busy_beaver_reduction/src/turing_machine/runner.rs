use rayon;
use tokio::sync::mpsc::Sender;

use log::{info, error};
use crate::turing_machine::turing_machine::TuringMachine;

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

        for mut turing_machine in turing_machines {
            let turing_machine_channel: Sender<TuringMachine> =
                self.tx_turing_machines.clone().unwrap();

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
        }

        info!("Finished running all the turing machines.");

        // after the running of every TuringMachine,
        // drop the communication channel with the database
        let _ = std::mem::replace(&mut self.tx_turing_machines, None);
        info!("Dropped communication channel betwenn Turing Machine and Database Manager runners.");
    }
}
