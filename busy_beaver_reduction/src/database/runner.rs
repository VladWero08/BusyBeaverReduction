use std::sync::mpsc::Receiver;

use crate::turing_machine::turing_machine::TuringMachine;

use super::manager::DatabaseManager;

pub struct DatabaseManagerRunner {
    database_manager: DatabaseManager,
    rx_turing_machines: Receiver<TuringMachine>,
}

impl DatabaseManagerRunner {
    pub async fn new(rx_turing_machines: Receiver<TuringMachine>) -> Self {
        let database_manager = DatabaseManager::new().await.unwrap();

        return DatabaseManagerRunner {
            database_manager: database_manager,
            rx_turing_machines: rx_turing_machines
        }
    }

    /// Listens to the communication channel, which has the TuringMachineRunner
    /// on the other side, and for each turing machine received, inserts it
    /// in the database.
    /// 
    /// Insert statements are made individual from the others.
    pub async fn receive_turing_machines(&mut self) {
        for turing_machine in self.rx_turing_machines.iter() {
            self.database_manager.insert_turing_machine(turing_machine).await;
        }
    }
}