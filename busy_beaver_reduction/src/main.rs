mod database;
mod delta;
mod filter;
mod generator;
mod logger;
mod mediator;
mod turing_machine;

use crate::logger::logger::load_logger;
use crate::mediator::mediator::Mediator;
use std::time::Instant;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    load_logger();

    let mut bb_mediator = Mediator::new(4);
    bb_mediator.load_turing_machines().await;

    match bb_mediator.get_loaded() {
        true => {
            bb_mediator.generate_and_filter().await;
            bb_mediator.run_and_update().await;
        }
        false => {
            bb_mediator.generate_and_filter().await;
            bb_mediator.run_and_insert().await;
        }
    }
}
