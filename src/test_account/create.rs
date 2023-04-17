use crate::{Error, Result};
use clap::Args;
use concordium_rust_sdk::{
    common::{
        self,
        types::{KeyIndex, KeyPair, TransactionTime},
    },
    endpoints::{Client as NodeClient, Endpoint},
    id::{
        constants::{ArCurve, AttributeKind, IpPairing},
        curve_arithmetic::{Curve, Value},
        dodis_yampolskiy_prf::SecretKey,
        identity_provider,
        pedersen_commitment::Randomness,
        types::{
            account_address_from_registration_id, AccountCredential, AccountCredentialMessage,
            AccountKeys, AttributeList, AttributeTag, CredentialData, CredentialPublicKeys, IpData,
            PublicCredentialData, PublicInformationForIp, SignatureThreshold, YearMonth,
        },
    },
    types::transactions::{BlockItem, Payload},
};
use rand::thread_rng;
use std::path::PathBuf;
use std::str::FromStr;

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

    #[arg(
        long,
        help = "Initial account valid_to attribute. This must be in YYYYMM format."
    )]
    valid_to: Option<String>,

    #[arg(long, help = "Destination account file path.")]
    output: Option<PathBuf>,
}

impl CreateTestAccountArgs {
    pub async fn run(self) -> Result<()> {
        let id_provider_data: IpData<IpPairing> = serde_json::from_str(
            &std::fs::read_to_string(&self.identity_provider).map_err(|_| {
                Error::new_invalid_argument_error(format!(
                    "could not read identity provider json file. path: {:?}",
                    &self.identity_provider
                ))
            })?,
        )
        .map_err(|_| {
            Error::new_invalid_argument_error(format!(
                "could not parse identity provider json file. path: {:?}",
                &self.identity_provider
            ))
        })?;

        let mut node_client = NodeClient::connect(self.node, "rpcadmin".to_string()).await?;
        let last_finalized_block_hash = node_client
            .get_consensus_status()
            .await?
            .last_finalized_block;
        let global_context = node_client
            .get_cryptographic_parameters(&last_finalized_block_hash)
            .await?;

        let created_at = YearMonth::now();
        let valid_to = if let Some(year_month_str) = self.valid_to.as_deref() {
            YearMonth::from_str(year_month_str).map_err(|_| {
                Error::new_invalid_argument_error(format!(
                    "invalid valid_to format. valid_to: {:?}",
                    year_month_str
                ))
            })?
        } else {
            YearMonth {
                year: created_at.year + 1,
                month: created_at.month,
            }
        };

        // define initial account attributes
        let attributes = AttributeList {
            valid_to,
            created_at,
            max_accounts: 100,
            alist: vec![
                (AttributeTag(0), AttributeKind("A".into())), // first name
                (AttributeTag(1), AttributeKind("B".into())), // last name
                (AttributeTag(2), AttributeKind("C".into())), // sex
                (AttributeTag(3), AttributeKind("D".into())), // dob
                (AttributeTag(4), AttributeKind("EE".into())), // country of residence
                (AttributeTag(5), AttributeKind("FFF".into())), // nationality
                (AttributeTag(6), AttributeKind("GGGG".into())), // id doc type
            ]
            .into_iter()
            .collect(),
            _phantom: Default::default(),
        };
        // build up initial account data
        let mut rng = thread_rng();
        let credential_data = CredentialData {
            keys: vec![(KeyIndex::from(0), KeyPair::generate(&mut rng))]
                .into_iter()
                .collect(),
            threshold: SignatureThreshold(1),
        };
        let prf_key = SecretKey::<ArCurve>::generate_non_zero(&mut rng);
        let cred_id_exponent = prf_key.prf_exponent(0).expect("We were very unlucky.");
        // RegId as well as Prf key commitments must be computed
        // with the same generators as in the commitment key.
        let cred_id = global_context
            .on_chain_commitment_key
            .hide(
                &Value::<ArCurve>::new(cred_id_exponent),
                &Randomness::zero(),
            )
            .0;

        let pub_info_for_ip = PublicInformationForIp {
            id_cred_pub: ArCurve::generate(&mut rng),
            reg_id: cred_id,
            vk_acc: CredentialPublicKeys {
                keys: credential_data.get_public_keys(),
                threshold: credential_data.get_threshold(),
            },
        };
        let address = account_address_from_registration_id(&pub_info_for_ip.reg_id);
        let expiry = TransactionTime::from_seconds((chrono::Utc::now().timestamp() + 3600) as u64);
        let initial_credential_deployment_info = identity_provider::create_initial_cdi(
            &id_provider_data.public_ip_info,
            pub_info_for_ip,
            &attributes,
            expiry,
            &id_provider_data.ip_cdi_secret_key,
        );
        let initial_account_keys = AccountKeys::from(credential_data);
        let block_item = BlockItem::<Payload>::from(AccountCredentialMessage {
            message_expiry: expiry,
            credential: AccountCredential::Initial {
                icdi: initial_credential_deployment_info,
            },
        });

        // compute destination
        let output_file = if let Some(output) = self.output {
            output
        } else {
            let mut output = std::env::current_dir().expect("must retrieve current directory...");
            output.push("accounts");
            output.push(format!("{}.json", address));
            output
        };
        if let Some(parent_dir) = output_file.parent() {
            if !parent_dir.exists() {
                println!("output directory created: {:?}", parent_dir);
                std::fs::create_dir_all(parent_dir)?;
            }
        }

        println!("let's send init account transaction for {}", address);
        let transaction_hash = node_client.send_block_item(&block_item).await?;
        println!("transaction_hash:{}", transaction_hash);

        serde_json::to_writer_pretty(
            std::fs::File::create(output_file)?,
            &serde_json::json!({"address": address, "keys": initial_account_keys, "encryptionSecretKey": common::base16_encode_string(&cred_id_exponent), "transactionHash": transaction_hash}),
        )?;

        Ok(())
    }
}
