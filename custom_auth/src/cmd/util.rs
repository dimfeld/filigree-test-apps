use clap::{Args, Subcommand};
use error_stack::{Report, ResultExt};
use schemars::schema_for;

use crate::Error;

#[derive(Args, Debug)]
pub struct UtilCommand {
    #[clap(subcommand)]
    pub command: UtilSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UtilSubcommand {}

impl UtilCommand {
    pub async fn handle(self) -> Result<(), Report<Error>> {
        match self.command {}

        Ok(())
    }
}
