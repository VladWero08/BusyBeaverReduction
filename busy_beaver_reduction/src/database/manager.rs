use log::{error, info};
use sqlx::query::Query;
use std::env;

use sqlx::mysql::{MySql, MySqlArguments, MySqlPoolOptions, MySqlQueryResult, MySqlRow};
use sqlx::{Pool, Row};

use crate::delta::transition_function::TransitionFunction;
use crate::turing_machine::turing_machine::TuringMachine;

const MAX_POOL_CONNECTIONS: u32 = 8;
const MAX_RETRIES: u8 = 3;

pub struct DatabaseManager {
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
                    return Some(DatabaseManager { pool: pool });
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

    /// Given a `MySqlRow1 object, that should contain
    /// an entry from the `turing_machines` table, transform
    /// it into a TuringMachine object.
    ///
    /// Returns the `TuringMachine` obtained.
    fn mysqlrow_to_turing_machine(&self, row: MySqlRow) -> TuringMachine {
        // reconstruct the transition function
        let transition_function_encoded = row.get(1);
        let number_of_states: i8 = row.get(2);
        let number_of_symbols: i8 = row.get(3);

        let mut transition_function =
            TransitionFunction::new(number_of_states as u8, number_of_symbols as u8);

        // decode the transition function
        transition_function.decode(transition_function_encoded);

        // reconstruct the turing machine
        let mut turing_machine = TuringMachine::new(transition_function);
        turing_machine.halted = row.get(4);
        turing_machine.steps = row.get(5);
        turing_machine.score = row.get(6);
        turing_machine.runtime = row.get(7);

        return turing_machine;
    }

    /// Given a number of states and a number of symbols,
    /// selects all the turing machines with a transtion functions
    /// that matches those numbers and `didn't halt`.
    ///
    /// Returns a `Option<Vec<TuringMachines>>` with all of them.
    pub async fn select_turing_machines_to_run(
        &mut self,
        number_of_states: u8,
        number_of_symbols: u8,
    ) -> Option<Vec<TuringMachine>> {
        let result: Result<Vec<MySqlRow>, sqlx::Error> = sqlx::query(
            "
                SELECT * 
                FROM turing_machines 
                WHERE number_of_states = ? 
                    AND number_of_symbols = ?
                    AND halted = FALSE",
        )
        .bind(number_of_states)
        .bind(number_of_symbols)
        .fetch_all(&self.pool)
        .await;

        match result {
            Ok(rows) => {
                let mut turing_machines = Vec::<TuringMachine>::new();

                for row in rows {
                    // reconstruct the turing machine
                    // from the mysqlrow
                    let turing_machine = self.mysqlrow_to_turing_machine(row);
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

    /// Given a turing machine, selects the turing machine
    /// from the database based on the encoding of the transition
    /// function.
    ///
    /// Returns the `id` of the entry in the database, `if the entry exists`.
    pub async fn select_turing_machine_by_delta(
        &mut self,
        turing_machine: &TuringMachine,
    ) -> Option<i32> {
        let transition_function_encoded = turing_machine.transition_function.encode();

        let result: Result<MySqlRow, sqlx::Error> = sqlx::query(
            "
                SELECT * 
                FROM turing_machines 
                WHERE transition_function = ?",
        )
        .bind(transition_function_encoded)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(row) => {
                return row.get(0);
            }
            Err(error) => {
                error!(
                    "While selecting a turing machine from database, by the transition function: {}",
                    error
                );
                return None;
            }
        }
    }

    /// Updates the turing machine in the database, if it
    /// actually exists in the database. The check is done
    /// using the `encoding` of the transition function.
    pub async fn update_turing_machine(&self, turing_machine: TuringMachine) {
        // encode the transition function as a string
        let transition_function_encoded = turing_machine.transition_function.encode();

        let result: Result<MySqlQueryResult, sqlx::Error> = sqlx::query(
            "
            UPDATE turing_machines
            SET halted = ?,
            steps = ?,
            score = ?,
            time_to_run = ?
            WHERE transition_function = ?
        ",
        )
        .bind(turing_machine.halted)
        .bind(turing_machine.steps)
        .bind(turing_machine.score)
        .bind(turing_machine.runtime)
        .bind(transition_function_encoded)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => {}
            Err(error) => {
                error!("While updating turing machine in the database: {}", error);
            }
        }
    }

    /// Inserts the given `TuringMachine` into the database.
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
    pub async fn batch_insert_turing_machines(&mut self, turing_machines: &[TuringMachine]) {
        // create and calculate the query statement
        let mut query_stmt = r#"
            INSERT INTO turing_machines 
            (transition_function, number_of_states, number_of_symbols, halted, steps, score, time_to_run) 
            VALUES
        "#.to_string();

        for _ in 0..turing_machines.len() - 1 {
            query_stmt += "(?, ?, ?, ?, ?, ?, ?),";
        }

        query_stmt += "(?, ?, ?, ?, ?, ?, ?)";

        // create the query for MySQL
        let mut query: Query<'_, MySql, MySqlArguments> = sqlx::query(query_stmt.as_str());

        // for each turing machine in the vector,
        // bind its values to the query
        for turing_machine in turing_machines {
            let transition_function_encoded = turing_machine.transition_function.encode();

            // a new query will be created after each
            // turing machine is added, that will stack them all up
            query = query
                .bind(transition_function_encoded)
                .bind(turing_machine.transition_function.number_of_states)
                .bind(turing_machine.transition_function.number_of_symbols)
                .bind(turing_machine.halted)
                .bind(turing_machine.steps)
                .bind(turing_machine.score)
                .bind(turing_machine.runtime);
        }

        let result = query.execute(&self.pool).await;

        match result {
            Ok(_) => {}
            Err(error) => {
                error!("While batch inserting multiple turing machines: {}", error);
            }
        }
    }
}
