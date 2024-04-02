mod database;
mod delta;
mod filter;
mod generator;
mod logger;
mod mediator;
mod turing_machine;

use log::info;

use crate::logger::logger::load_logger;
use crate::mediator::mediator::Mediator;

#[tokio::main]
async fn main() {
    load_logger();

    let bb_mediator = Mediator::new(2);
    bb_mediator.generate_and_filter().await;
}
