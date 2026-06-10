use clap::{Parser, Subcommand};
use log::info;

pub mod cli;
pub mod commands;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();

    info!("ArduinoConnector initalized successfuly!");

    let subcommand = cli.command.as_runnable();

    subcommand.run()?;

    Ok(())
}
