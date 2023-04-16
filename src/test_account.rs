mod create;
pub use create::*;

use crate::Result;
use clap::Subcommand;

/// Subcommand for `test-account`
#[derive(Subcommand)]
pub enum TestAccountSubcommand {
    Create(CreateTestAccountArgs),
}

impl TestAccountSubcommand {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::Create(args) => args.run().await,
        }
    }
}
