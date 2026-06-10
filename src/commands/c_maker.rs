use log::info;

use crate::cli::{CMakerArgs, DevicesArgs};
use crate::commands::Run;

impl Run for CMakerArgs {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
