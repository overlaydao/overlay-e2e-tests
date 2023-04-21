use crate::{Error, Result};
use clap::Args;
use concordium_rust_sdk::{
    common,
    types::{smart_contracts::WasmModule, WalletAccount},
    v2::{Client as NodeClient, Endpoint},
};
use std::{io::Cursor, path::PathBuf};

#[derive(Args)]
pub struct InitContractArgs {
    #[arg(
        long,
        help = "GRPC interface of the node.",
        default_value = "http://localhost:11000"
    )]
    node: Endpoint,

    #[arg(long, help = "Path to the account key file.")]
    account: PathBuf,

    #[clap(
        long,
        help = "Path to smart contract module binaries build in wasm format."
    )]
    wasm: PathBuf,
}

impl InitContractArgs {
    pub async fn run(self) -> Result<()> {
        // parse wallet account file.
        let wallet_file_content = std::fs::read_to_string(&self.account).map_err(|_| {
            Error::new_invalid_argument_error(format!(
                "could not read account json file. path: {:?}",
                &self.account
            ))
        })?;
        let from_account = WalletAccount::from_json_str(&wallet_file_content).map_err(|err| {
            eprintln!("{:?}", err);
            Error::new_invalid_argument_error(format!(
                "could not parse account json file. path: {:?}",
                &self.account
            ))
        })?;

        // parse wasm file.
        let wasm_module_content = std::fs::read(&self.wasm).map_err(|_| {
            Error::new_invalid_argument_error(format!(
                "could not read wasm file. path: {:?}",
                &self.wasm
            ))
        })?;
        let mut cursor = Cursor::new(wasm_module_content);
        let wasm_module: WasmModule = common::from_bytes(&mut cursor)?;

        // init concordium node grpc v2 client
        let mut node_client = NodeClient::new(self.node).await?;

        // TODO check module already exists or not.
        // TODO deploy module
        todo!()
    }
}
