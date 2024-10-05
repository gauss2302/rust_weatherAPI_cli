mod api_client;
mod cli;
mod model;
mod weather_service;

use crate::cli::Cli;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let api_key =
        env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set in .env file");

    let cli = Cli::new(api_key);
    cli.run();
}
