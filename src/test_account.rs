mod create;
mod get;
mod send_ccd;

pub use create::*;
pub use get::*;
pub use send_ccd::*;

use crate::Result;
use clap::Subcommand;

/// Subcommand for `test-account`
#[derive(Subcommand)]
pub enum TestAccountSubcommand {
    Create(CreateTestAccountArgs),
    Get(GetTestAccountInfoArgs),
    SendCCD(SendCCDArgs),
}

impl TestAccountSubcommand {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::Create(args) => args.run().await,
            Self::Get(args) => args.run().await,
            Self::SendCCD(args) => args.run().await,
        }
    }
}
