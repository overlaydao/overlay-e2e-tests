use crate::{Error, Result};
use clap::Args;

#[derive(Args)]
pub struct InitContractArgs {}

impl InitContractArgs {
    pub async fn run(self) -> Result<()> {
        todo!()
    }
}
