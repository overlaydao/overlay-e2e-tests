use crate::Result;
use clap::Args;
use concordium_rust_sdk::{
    id::types::AccountAddress,
    types::hashes::BlockHash,
    v2::{BlockIdentifier, Client as NodeClient, Endpoint},
};

#[derive(Args)]
pub struct GetTestAccountInfoArgs {
    #[arg(
        long,
        help = "GRPC interface of the node.",
        default_value = "http://localhost:11000"
    )]
    node: Endpoint,

    #[arg(long, help = "Account address to query.")]
    address: AccountAddress,

    #[arg(long, help = "Block to query the account in.")]
    block: Option<BlockHash>,
}

impl GetTestAccountInfoArgs {
    pub async fn run(self) -> Result<()> {
        let block_ident = self
            .block
            .map_or(BlockIdentifier::LastFinal, BlockIdentifier::Given);
        let mut node_client = NodeClient::new(self.node).await?;
        let ai = node_client
            .get_account_info(&self.address.into(), &block_ident)
            .await?;
        println!("{:#?}", ai);
        Ok(())
    }
}
