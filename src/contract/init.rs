mod supported_contracts;
use supported_contracts::{overlay_projects, ContractName};

use crate::{Error, Result};
use anyhow::anyhow;
use clap::Args;
use concordium_rust_sdk::{
    common::{
        self,
        types::{Amount, TransactionTime},
    },
    types::{
        smart_contracts::{ModuleReference, OwnedContractName, OwnedParameter, WasmModule},
        transactions, AccountTransactionEffects, BlockItemSummaryDetails, ContractAddress,
        RejectReason, TransactionType, WalletAccount,
    },
    v2::{Client as NodeClient, Endpoint},
};
use std::{io::Cursor, path::PathBuf, str::FromStr};
use tracing::{error, info};

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

    #[arg(
        long,
        help = "Path to smart contract module binaries build in wasm format."
    )]
    wasm: PathBuf,

    #[arg(long, help = "Contract name to init.")]
    name: String,

    #[arg(long, help = "Path to init params build in json format.")]
    params: Option<PathBuf>,
}

impl InitContractArgs {
    pub async fn run(self) -> Result<()> {
        // check name & params
        let contract_name = ContractName::from_str(&self.name)?;
        let param = if let Some(params_file_path) = &self.params {
            let params_file_content = std::fs::read_to_string(params_file_path).map_err(|_| {
                Error::new_invalid_argument_error(format!(
                    "could not read params json file. path: {:?}",
                    params_file_path
                ))
            })?;
            match contract_name {
                ContractName::OverlayUsers => OwnedParameter::default(),
                ContractName::OverlayProjects => {
                    let params: overlay_projects::UpdateContractStateParams =
                        serde_json::from_str(&params_file_content).map_err(|_| {
                            Error::new_invalid_argument_error(format!(
                                "could not parse params json file. path: {:?}",
                                params_file_path
                            ))
                        })?;
                    let params_bytes = common::to_bytes(&params);
                    params_bytes.try_into().map_err(|_| {
                        Error::new_invalid_argument_error(format!(
                            "could not parse params json value to init parameters. path: {:?}",
                            params_file_path
                        ))
                    })?
                },
                ContractName::OverlaySales => todo!(),
            }
        } else {
            OwnedParameter::default()
        };

        // parse wallet account file.
        let wallet_file_content = std::fs::read_to_string(&self.account).map_err(|_| {
            Error::new_invalid_argument_error(format!(
                "could not read account json file. path: {:?}",
                &self.account
            ))
        })?;
        let from_account = WalletAccount::from_json_str(&wallet_file_content).map_err(|err| {
            error!("{:?}", err);
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

        /* deploy module if not exists */
        // check module already exists or not.
        let exists = module_exists(&mut node_client, &wasm_module).await?;
        let module_ref = if exists {
            // if module already exists, then just print.
            let module_ref = wasm_module.get_module_ref();
            info!("Module with reference {} already exists.", module_ref);
            module_ref
        } else {
            // deploy module to the chain.
            deploy_module(&mut node_client, &from_account, wasm_module).await?
        };

        /* init contract */
        let init_name = OwnedContractName::new(format!("init_{}", contract_name.as_str())).unwrap();
        let payload = transactions::InitContractPayload {
            init_name,
            amount: Amount::from_micro_ccd(0),
            mod_ref: module_ref,
            param,
        };
        let contract_address = init_contract(&mut node_client, payload).await?;
        println!(
            "Initialized contract, address: ({}, {})",
            contract_address.index, contract_address.subindex
        );
        Ok(())
    }
}

async fn module_exists(node_client: &mut NodeClient, wasm_module: &WasmModule) -> Result<bool> {
    let consensus_info = node_client.get_consensus_info().await?;
    let latest_block = consensus_info.last_finalized_block;
    let module_ref = wasm_module.get_module_ref();
    let module_ref = node_client
        .get_module_source(&module_ref, &latest_block)
        .await;
    match module_ref {
        Ok(_) => Ok(true),
        Err(e) if e.is_not_found() => Ok(false),
        Err(e) => Err(e.into()),
    }
}

async fn deploy_module(
    node_client: &mut NodeClient,
    from_account: &WalletAccount,
    wasm_module: WasmModule,
) -> Result<ModuleReference> {
    let nonce = node_client
        .get_next_account_sequence_number(&from_account.address)
        .await?;
    if !nonce.all_final {
        return Err(Error::SystemError {
            cause: anyhow!(
                "There are unfinalized transactions. Transaction nonce is not reliable enough."
            ),
        });
    }
    let expiry = TransactionTime::from_seconds((chrono::Utc::now().timestamp() + 300) as u64);
    let tx = transactions::send::deploy_module(
        from_account,
        from_account.address,
        nonce.nonce,
        expiry,
        wasm_module,
    );
    let block_item = transactions::BlockItem::AccountTransaction(tx);
    let tx_hash = node_client.send_block_item(&block_item).await?;
    let (_, block_item) = node_client.wait_until_finalized(&tx_hash).await?;
    let module_ref = match block_item.details {
        BlockItemSummaryDetails::AccountTransaction(account_transaction_details) => {
            match account_transaction_details.effects {
                AccountTransactionEffects::None {
                    transaction_type,
                    reject_reason,
                } => {
                    if transaction_type != Some(TransactionType::DeployModule) {
                        return Err(Error::SystemError {
                            cause: anyhow!(
                                "Expected transaction type to be DeployModule if rejected."
                            ),
                        });
                    }
                    match reject_reason {
                        RejectReason::ModuleHashAlreadyExists { contents } => Ok(contents),
                        _ => Err(Error::SystemError {
                            cause: anyhow!("module deploy rejected with reason: {reject_reason:?}"),
                        }),
                    }
                },
                AccountTransactionEffects::ModuleDeployed { module_ref } => Ok(module_ref),
                _ => Err(Error::SystemError {
                    cause: anyhow!("invalid transaction effects"),
                }),
            }
        },
        _ => Err(Error::SystemError {
            cause: anyhow!("Expected Account transaction"),
        }),
    }?;
    info!(
        "Transaction finalized, tx_hash={} module_ref={}",
        tx_hash, module_ref,
    );
    Ok(module_ref)
}

async fn init_contract(
    _node_client: &mut NodeClient,
    _payload: transactions::InitContractPayload,
) -> Result<ContractAddress> {
    todo!()
}
