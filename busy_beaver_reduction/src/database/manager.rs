use dotenv::dotenv;
use log::{error, info};
use std::env;

use sqlx::mysql::{MySql, MySqlPoolOptions, MySqlQueryResult, MySqlRow};
use sqlx::{Pool, Row};

use crate::delta::transition_function::TransitionFunction;
use crate::turing_machine::turing_machine::TuringMachine;

const MAX_POOL_CONNECTIONS: u32 = 8;
const MAX_RETRIES: u8 = 3;

pub struct DatabaseManager {
    connection_string: String,
    pool: Pool<MySql>,
}

impl DatabaseManager {
    pub async fn new() -> Option<Self> {
        // counter for the number of times tried to connect
        // to the database
        let mut connection_retries: u8 = 0;

        while connection_retries < MAX_RETRIES {
            let connection_string = DatabaseManager::get_connection_string();
            let pool = DatabaseManager::get_pool(&connection_string).await;

            match pool {
                Ok(pool) => {
                    info!("DatabaseManager created successfully!");
                    return Some(DatabaseManager {
                        connection_string: connection_string,
                        pool: pool,
                    });
                }
                Err(error) => {
                    error!("DatabaseManager couldn't be created: {}", error);
                }
            }

            // increase the number of tries
            connection_retries += 1;
        }

        return None;
    }

    /// Loads and gets the `connection string` to the database,
    /// from the `.env` file configured in the crate.
    fn get_connection_string() -> String {
        dotenv().ok();

        match env::var("DATABASE_URL") {
            Ok(connection_string) => {
                return connection_string.to_string();
            }

            Err(error) => {
                error!(
                    "While setting the connection string for the database: {}",
                    error
                );
                return "".to_string();
            }
        }
    }

    /// Gets the `pool` of connections using the `connection_string`.
    async fn get_pool(connection_string: &String) -> Result<Pool<MySql>, sqlx::Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(MAX_POOL_CONNECTIONS)
            .connect(&connection_string)
            .await?;

        Ok(pool)
    }

    /// Given a number of states and a number of symbols,
    /// selects all the turing machines with a transtion functions
    /// that matches those numbers, and returns a `Vec<TuringMachines>`
    /// with all of them.
    pub async fn select_turing_machines(
        &mut self,
        number_of_states: u8,
        number_of_symbols: u8,
    ) -> Option<Vec<TuringMachine>> {
        let result: Result<Vec<MySqlRow>, sqlx::Error> = sqlx::query(
            "
                SELECT * 
                FROM turing_machines 
                WHERE number_of_states = ? 
                    AND number_of_symbols = ?",
        )
        .bind(number_of_states)
        .bind(number_of_symbols)
        .fetch_all(&self.pool)
        .await;

        match result {
            Ok(rows) => {
                let mut turing_machines = Vec::<TuringMachine>::new();

                for row in rows {
                    // reconstruct the transition function
                    let transition_function_encoded = row.get(1);

                    let mut transition_function =
                        TransitionFunction::new(number_of_states, number_of_symbols);
                    // decode the transition function
                    transition_function.decode(transition_function_encoded);

                    // reconstruct the turing machine
                    let mut turing_machine = TuringMachine::new(transition_function);
                    turing_machine.halted = row.get(4);
                    turing_machine.steps = row.get(5);
                    turing_machine.score = row.get(6);
                    turing_machine.runtime = row.get(7);

                    // increase the vector of turing machines from the database
                    turing_machines.push(turing_machine);
                }

                return Some(turing_machines);
            }
            Err(error) => {
                error!(
                    "While selecting all turing machines from database: {}",
                    error
                );
                return None;
            }
        }
    }

    /// Using the `pool` of connections, insert the given `TuringMachine`
    /// into the `turing_machines` table.
    pub async fn insert_turing_machine(&mut self, turing_machine: TuringMachine) {
        // get the encoding of the transition function, as a string,
        // so it is valid for insert in the database
        let transition_function_encoded = turing_machine.transition_function.encode();

        let result: Result<MySqlQueryResult, sqlx::Error> = sqlx::query("
            INSERT INTO turing_machines 
            (transition_function, number_of_states, number_of_symbols, halted, steps, score, time_to_run) 
            VALUES
            (?, ?, ?, ?, ?, ?, ?)")
            .bind(transition_function_encoded)
            .bind(turing_machine.transition_function.number_of_states)
            .bind(turing_machine.transition_function.number_of_symbols)
            .bind(turing_machine.halted)
            .bind(turing_machine.steps)
            .bind(turing_machine.score)
            .bind(turing_machine.runtime)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => {}
            Err(error) => {
                error!("While inserting turing machine in the database: {}", error);
            }
        }
    }

    /// Using the `pool` of connections, insert the given vector of
    /// `TuringMachine`s into the `turing machines` table.
    ///
    /// A batch insert will be made with all of them.
    pub async fn batch_insert_turing_machines(&mut self, turing_machines: Vec<TuringMachine>) {
        // TO DO
    }
}
