mod init;

pub use init::*;

use crate::Result;
use clap::Subcommand;

/// Subcommand for `test-account`
#[derive(Subcommand)]
pub enum ContractSubcommand {
    Init(InitContractArgs),
}

impl ContractSubcommand {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::Init(args) => args.run().await,
        }
    }
}
