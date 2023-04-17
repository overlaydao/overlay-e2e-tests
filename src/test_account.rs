mod create;
mod get;

pub use create::*;
pub use get::*;

use crate::Result;
use clap::Subcommand;

/// Subcommand for `test-account`
#[derive(Subcommand)]
pub enum TestAccountSubcommand {
    Create(CreateTestAccountArgs),
    Get(GetTestAccountInfoArgs),
}

impl TestAccountSubcommand {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::Create(args) => args.run().await,
            Self::Get(args) => args.run().await,
        }
    }
}
