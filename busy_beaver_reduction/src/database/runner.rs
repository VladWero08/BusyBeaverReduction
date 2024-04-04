use tokio::sync::mpsc::Receiver;

use super::manager::DatabaseManager;
use crate::turing_machine::turing_machine::TuringMachine;

pub struct DatabaseManagerRunner {
    rx_turing_machines: Receiver<TuringMachine>,
}

impl DatabaseManagerRunner {
    pub fn new(rx_turing_machines: Receiver<TuringMachine>) -> Self {
        DatabaseManagerRunner { rx_turing_machines }
    }

    /// Listens to the communication channel, which has the TuringMachineRunner
    /// on the other side, and for each turing machine received, inserts it
    /// in the database.
    ///
    /// Insert statements are made individual from the others.
    pub async fn receive_turing_machines(&mut self) {
        let database = match DatabaseManager::new().await {
            Some(database) => database,
            None => return,
        };

        // wait for every turing machine executed to come
        // and then update its entry in the database
        while let Some(turing_machine) = self.rx_turing_machines.recv().await {
            database.update_turing_machine(turing_machine).await;
        }
    }
}
