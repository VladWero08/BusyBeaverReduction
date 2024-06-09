use rayon;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Semaphore, SemaphorePermit};

use crate::filter::filter_runtime::FilterRuntimeType;
use crate::turing_machine::turing_machine::TuringMachine;
use log::{error, info};

const MAXIMUM_THREADS: usize = 8;

pub struct TuringMachineRunner {
    pub tx_turing_machines: Option<Sender<TuringMachine>>,
    pub short_escapers: i64,
    pub long_escapers: i64,
    pub cyclers: i64,
    pub translated_cyclers: i64,
}

impl TuringMachineRunner {
    pub fn new(tx_turing_machine: Sender<TuringMachine>) -> Self {
        TuringMachineRunner {
            tx_turing_machines: Some(tx_turing_machine),
            short_escapers: 0,
            long_escapers: 0,
            cyclers: 0,
            translated_cyclers: 0,
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
    pub async fn run(&mut self, mut turing_machines: Vec<TuringMachine>) {
        info!(
            "Started running turing machine. {} total machines to run...",
            turing_machines.len()
        );

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(MAXIMUM_THREADS)
            .build()
            .unwrap();

        pool.install(|| {
            turing_machines.par_iter_mut().for_each(|turing_machine| {
                turing_machine.execute();
            });
        });

        // counter for the number of Turing machines that did not halt
        let mut non_halting_turing_machines_size: i64 = 0;

        for turing_machine in turing_machines {
            // check if the machines was fileted
            match turing_machine.filtered {
                FilterRuntimeType::ShortEscapee => self.short_escapers += 1,
                FilterRuntimeType::LongEscapee => self.long_escapers += 1,
                FilterRuntimeType::Cycler => self.cyclers += 1,
                FilterRuntimeType::TranslatedCycler => self.translated_cyclers += 1,
                FilterRuntimeType::None => {}
            }

            if turing_machine.halted == false {
                non_halting_turing_machines_size += 1;
            }

            let turing_machine_channel: Sender<TuringMachine> =
                self.tx_turing_machines.clone().unwrap();
            let _ = turing_machine_channel.send(turing_machine).await;
        }

        self.display_filtering_results(non_halting_turing_machines_size);

        // after the running of every TuringMachine,
        // drop the communication channel with the database
        let _ = std::mem::replace(&mut self.tx_turing_machines, None);
        info!("Dropped communication channel betwenn Turing Machine and Database Manager runners.");
    }

    /// Older version used to run all the Turing machines. It is deprecated
    /// because it created a big overhead with all the threads created.
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

    pub fn display_filtering_results(&self, turing_machines_size: i64) {
        let short_escapers_percentage =
            self.short_escapers as f64 * 100.0 / turing_machines_size as f64;
        let long_escapers_percentage =
            self.long_escapers as f64 * 100.0 / turing_machines_size as f64;
        let cyclers_percentage = self.cyclers as f64 * 100.0 / turing_machines_size as f64;
        let translated_cyclers_percentage =
            self.translated_cyclers as f64 * 100.0 / turing_machines_size as f64;

        let total = short_escapers_percentage
            + long_escapers_percentage
            + cyclers_percentage
            + translated_cyclers_percentage;

        info!(
            "Filtered a total of short escapers: {:.2}%",
            short_escapers_percentage
        );

        info!(
            "Filtered a total of long escapers: {:.2}%",
            long_escapers_percentage
        );

        info!("Filtered a total of cyclers: {:.2}%", cyclers_percentage);

        info!(
            "Filtered a total of translated cyclers: {:.2}%",
            translated_cyclers_percentage
        );

        info!(
            "Filtered a total of {:.2}% Turing machines HOLDOUTS with runtime filters.",
            total
        );
    }
}
