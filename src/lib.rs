mod test_account;

mod error;
use error::Error;
type Result<T> = core::result::Result<T, Error>;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub enum Cli {
    /// operations related with test accounts
    #[command(subcommand)]
    TestAccount(test_account::TestAccountSubcommand),
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::TestAccount(subcommand) => subcommand.run().await,
        }
    }
}
