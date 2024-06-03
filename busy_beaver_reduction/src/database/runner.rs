use tokio::sync::mpsc::Receiver;

use super::manager::DatabaseManager;
use crate::turing_machine::turing_machine::TuringMachine;

const BATCH_SIZE: usize = 1000;

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
    /// Update statements are made individual from the others.
    pub async fn receive_and_update_turing_machines(&mut self) {
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

    /// Listens to the communication channel, which has the TuringMachineRunner
    /// on the other side, and for each turing machine received, add it to a
    /// vector of Turing machines. 
    /// 
    /// Once the desired batch size is reached, bulks insert them in the database.
    pub async fn receive_and_insert_turing_machines(&mut self) {
        let mut database = match DatabaseManager::new().await {
            Some(database) => database,
            None => return,
        };
        let mut turing_machines: Vec<TuringMachine> = Vec::new();

        // wait for every turing machine executed to come
        // and then update its entry in the database
        while let Some(turing_machine) = self.rx_turing_machines.recv().await {
            turing_machines.push(turing_machine);

            if turing_machines.len() == BATCH_SIZE {
                database.batch_insert_turing_machines(&turing_machines[..]).await;
                turing_machines = Vec::new();
            }
        }
    }
}
