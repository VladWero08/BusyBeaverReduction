mod database;
mod delta;
mod filter;
mod generator;
mod logger;
mod mediator;
mod turing_machine;

use crate::logger::logger::load_logger;
use crate::mediator::mediator::Mediator;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    load_logger();

    let mut bb_mediator = Mediator::new(3);
    bb_mediator.load_turing_machines().await;

    match bb_mediator.loaded {
        true => {
            bb_mediator.run_and_update().await;
        }
        false => {
            bb_mediator.generate_and_filter().await;
            bb_mediator.run_and_insert().await;
        }
    }
}
