use crate::Result;
use clap::Args;
use concordium_rust_sdk::endpoints::Endpoint;
use std::path::PathBuf;

#[derive(Args)]
pub struct CreateTestAccountArgs {
    #[arg(
        long,
        help = "GRPC interface of the node.",
        default_value = "http://localhost:10000"
    )]
    node: Endpoint,

    #[arg(
        long,
        help = "Identity provider json file such as 'idps/ip-data-0.json'."
    )]
    identity_provider: PathBuf,
}

impl CreateTestAccountArgs {
    pub async fn run(self) -> Result<()> {
        todo!()
    }
}
