mod contract;
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
    /// operations related with smart contracts
    #[command(subcommand)]
    Contract(contract::ContractSubcommand),
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::TestAccount(subcommand) => subcommand.run().await,
            Self::Contract(subcommand) => subcommand.run().await,
        }
    }
}
