use crate::{Error, Result};
use clap::Args;
use concordium_rust_sdk::{
    common::types::{Amount, TransactionTime},
    id::types::AccountAddress,
    types::{transactions, Memo, WalletAccount},
    v2::{BlockIdentifier, Client as NodeClient, Endpoint},
};
use std::path::PathBuf;

#[derive(Args)]
pub struct SendCCDArgs {
    #[arg(
        long,
        help = "GRPC interface of the node.",
        default_value = "http://localhost:11000"
    )]
    node: Endpoint,

    #[arg(long, help = "Path to the account key file.")]
    from: PathBuf,

    #[arg(long, help = "Amount to send in micro CCD.")]
    amount: u64,

    #[arg(long, help = "Account address to send to.")]
    to: AccountAddress,

    #[arg(long, help = "Optional memo to be included in the transaction.")]
    memo: Option<String>,
}

impl SendCCDArgs {
    pub async fn run(self) -> Result<()> {
        let wallet_file_content = std::fs::read_to_string(&self.from).map_err(|_| {
            Error::new_invalid_argument_error(format!(
                "could not read account json file. path: {:?}",
                &self.from
            ))
        })?;
        let from_account = WalletAccount::from_json_str(&wallet_file_content).map_err(|err| {
            eprintln!("{:?}", err);
            Error::new_invalid_argument_error(format!(
                "could not parse account json file. path: {:?}",
                &self.from
            ))
        })?;

        let amount = Amount::from_micro_ccd(self.amount);
        let memo = if let Some(memo) = self.memo.as_deref() {
            println!("Sending transfer with memo: \"{}\"", memo);
            let memo: Memo = memo.as_bytes().to_owned().try_into().map_err(|_| {
                Error::new_invalid_argument_error(format!(
                    "invalid memo (may be too long): {}",
                    memo
                ))
            })?;
            Some(memo)
        } else {
            println!("Sending transfer");
            None
        };

        let mut node_client = NodeClient::new(self.node).await?;
        let ai = node_client
            .get_account_info(&from_account.address.into(), &BlockIdentifier::Best)
            .await?
            .response;
        let nonce = ai.account_nonce;

        // set expiry to now + 5min
        let expiry: TransactionTime =
            TransactionTime::from_seconds((chrono::Utc::now().timestamp() + 300) as u64);
        let tx = if let Some(memo) = memo {
            transactions::send::transfer_with_memo(
                &from_account,
                from_account.address,
                nonce,
                expiry,
                self.to,
                amount,
                memo,
            )
        } else {
            transactions::send::transfer(
                &from_account,
                from_account.address,
                nonce,
                expiry,
                self.to,
                amount,
            )
        };

        let transaction_hash = node_client.send_account_transaction(tx).await?;
        println!(
            "Transaction {} submitted (nonce = {}).",
            transaction_hash, nonce,
        );
        println!("Waiting until finalized.");
        let (bh, bs) = node_client.wait_until_finalized(&transaction_hash).await?;
        println!("Transaction finalized in block {}.", bh);
        println!("The outcome is {:#?}", bs);

        Ok(())
    }
}
