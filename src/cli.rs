use clap::{Args, Command, Parser, Subcommand};

use crate::commands::Run;

#[derive(Parser, Debug)]
#[command(name = "ardConnector")]
#[command(about = "Arduino connector - simple tool to talk to boards outside arduino IDE!")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    CMaker(CMakerArgs),
    Devices(DevicesArgs),
}

impl Commands {
    pub fn as_runnable(&self) -> &dyn Run {
        match self {
            Commands::CMaker(args) => args,
            Commands::Devices(args) => args,
        }
    }
}

#[derive(Args, Debug)]
pub struct CMakerArgs {}

#[derive(Args, Debug)]
pub struct DevicesArgs {}
