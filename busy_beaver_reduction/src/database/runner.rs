use std::thread;
use std::sync::mpsc::Receiver;

use sqlx::database;

use crate::turing_machine::turing_machine::TuringMachine;
use super::manager::DatabaseManager;

pub struct DatabaseManagerRunner {
    rx_turing_machines: Receiver<TuringMachine>,
}

impl DatabaseManagerRunner {
    pub fn new(rx_turing_machines: Receiver<TuringMachine>) -> Self {
        DatabaseManagerRunner {
            rx_turing_machines
        }
    }

    /// Listens to the communication channel, which has the TuringMachineRunner
    /// on the other side, and for each turing machine received, inserts it
    /// in the database.
    ///
    /// Insert statements are made individual from the others.
    pub fn receive_turing_machines(&mut self) {
        for turing_machine in self.rx_turing_machines.iter() {
            tokio::spawn(async {
                let database = DatabaseManager::new().await;

                match database {
                    Some(mut database) => {
                        database.insert_turing_machine(turing_machine).await;
                    }
                    None => {}
                }
            });
        }
    }
}
