use crate::{Error, Result};
use clap::Args;
use concordium_rust_sdk::smart_contracts::common::AccountAddress;
use concordium_rust_sdk::{
    common::types::{KeyIndex, KeyPair, TransactionTime},
    id::{
        account_holder,
        constants::{ArCurve, AttributeKind, IpPairing},
        curve_arithmetic::{Curve, Value},
        dodis_yampolskiy_prf::SecretKey,
        identity_provider,
        pedersen_commitment::Randomness,
        ps_sig,
        secret_sharing::Threshold,
        types::{
            account_address_from_registration_id, AccCredentialInfo, AccountCredential,
            AccountCredentialMessage, AccountKeys, ArData, AttributeList, AttributeTag,
            CredentialData, CredentialHolderInfo, CredentialPublicKeys, IdCredentials,
            IdObjectUseData, IdentityObject, InitialAccountData, IpContext, IpData, Policy,
            PublicCredentialData, PublicInformationForIp, SignatureThreshold,
            SystemAttributeRandomness, YearMonth,
        },
    },
    types::{
        transactions::{BlockItem, Payload},
        CryptographicParameters,
    },
    v2::{BlockIdentifier, Client as NodeClient, Endpoint},
};
use either::Either;
use rand::{thread_rng, CryptoRng, Rng};
use serde::Serialize;
use std::{collections::BTreeMap, path::PathBuf, str::FromStr};

#[derive(Args)]
pub struct CreateTestAccountArgs {
    #[arg(
        long,
        help = "GRPC interface of the node.",
        default_value = "http://localhost:11000"
    )]
    node: Endpoint,

    #[arg(
        long,
        help = "Identity provider json file such as 'idps/ip-data-0.json'."
    )]
    identity_provider: PathBuf,

    #[arg(
        long,
        help = "Anonymity revoker json file such as 'ars/ar-data-1.json'."
    )]
    anonymity_revoker: PathBuf,

    #[arg(long, help = "Create initial account only flag.")]
    initial_account: bool,

    #[arg(
        long,
        help = "Initial account valid_to attribute. This must be in YYYYMM format."
    )]
    valid_to: Option<String>,

    #[arg(long, help = "Destination account file path.")]
    output: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct CreateAccountOutputs {
    address: AccountAddress,
    keys: AccountKeys,
}

impl CreateTestAccountArgs {
    pub async fn run(self) -> Result<()> {
        let mut csprng = thread_rng();
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
        let anonymity_revoker_data: ArData<ArCurve> = serde_json::from_str(
            &std::fs::read_to_string(&self.anonymity_revoker).map_err(|_| {
                Error::new_invalid_argument_error(format!(
                    "could not read anonymity revoker json file. path: {:?}",
                    &self.anonymity_revoker
                ))
            })?,
        )
        .map_err(|_| {
            Error::new_invalid_argument_error(format!(
                "could not parse anonymity revoker json file. path: {:?}",
                &self.anonymity_revoker
            ))
        })?;

        let mut node_client = NodeClient::new(self.node.clone()).await?;
        let global_context = node_client
            .get_cryptographic_parameters(&BlockIdentifier::Best)
            .await?
            .response;
        let account = if self.initial_account {
            self.create_init_account(
                &mut node_client,
                &id_provider_data,
                &global_context,
                &mut csprng,
            )
            .await?
        } else {
            self.create_normal_account(
                &mut node_client,
                &id_provider_data,
                &anonymity_revoker_data,
                &global_context,
                &mut csprng,
            )
            .await?
        };

        // output WalletAccount json file
        let output_file = if let Some(output) = self.output {
            output
        } else {
            let mut output = std::env::current_dir().expect("must retrieve current directory...");
            output.push("accounts");
            output.push(format!("{}.json", account.address));
            output
        };
        if let Some(parent_dir) = output_file.parent() {
            if !parent_dir.exists() {
                println!("output directory created: {:?}", parent_dir);
                std::fs::create_dir_all(parent_dir)?;
            }
        }
        serde_json::to_writer_pretty(
            std::fs::File::create(output_file)?,
            &serde_json::json!(account),
        )?;
        Ok(())
    }

    fn get_valid_to(&self) -> Result<YearMonth> {
        let valid_to = if let Some(year_month_str) = self.valid_to.as_deref() {
            YearMonth::from_str(year_month_str).map_err(|_| {
                Error::new_invalid_argument_error(format!(
                    "invalid valid_to format. valid_to: {:?}",
                    year_month_str
                ))
            })?
        } else {
            let now = YearMonth::now();
            YearMonth {
                year: now.year + 1,
                month: now.month,
            }
        };
        Ok(valid_to)
    }

    fn get_attributes(&self) -> Result<AttributeList<<ArCurve as Curve>::Scalar, AttributeKind>> {
        let created_at = YearMonth::now();
        let valid_to = self.get_valid_to()?;
        Ok(AttributeList {
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
        })
    }

    async fn create_init_account<T: CryptoRng + Rng>(
        &self,
        node_client: &mut NodeClient,
        id_provider_data: &IpData<IpPairing>,
        global_context: &CryptographicParameters,
        csprng: &mut T,
    ) -> Result<CreateAccountOutputs> {
        // define initial account attributes
        let attributes = self.get_attributes()?;
        // build up initial account data
        let credential_data = CredentialData {
            keys: vec![(KeyIndex::from(0), KeyPair::generate(csprng))]
                .into_iter()
                .collect(),
            threshold: SignatureThreshold(1),
        };
        let prf_key = SecretKey::<ArCurve>::generate_non_zero(csprng);
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
            id_cred_pub: ArCurve::generate(csprng),
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
        let transaction = BlockItem::<Payload>::from(AccountCredentialMessage {
            message_expiry: expiry,
            credential: AccountCredential::Initial {
                icdi: initial_credential_deployment_info,
            },
        });
        println!("let's send init account transaction for {}", address);
        let transaction_hash = node_client.send_block_item(&transaction).await?;
        println!("transaction_hash:{}", transaction_hash);
        let (block_hash, block_item_summary) =
            node_client.wait_until_finalized(&transaction_hash).await?;
        println!("Transaction finalized in block {}.", block_hash);
        println!("The outcome is {:#?}", block_item_summary);
        Ok(CreateAccountOutputs {
            address,
            keys: AccountKeys::from(credential_data),
        })
    }

    async fn create_normal_account<T: CryptoRng + Rng>(
        &self,
        node_client: &mut NodeClient,
        id_provider_data: &IpData<IpPairing>,
        anonymity_revoker_data: &ArData<ArCurve>,
        global_context: &CryptographicParameters,
        csprng: &mut T,
    ) -> Result<CreateAccountOutputs> {
        // define initial account attributes
        let attributes = self.get_attributes()?;
        let mut ars_infos = BTreeMap::new();
        ars_infos.insert(
            anonymity_revoker_data.public_ar_info.ar_identity,
            anonymity_revoker_data.public_ar_info.clone(),
        );
        let ip_context = IpContext::new(
            &id_provider_data.public_ip_info,
            &ars_infos,
            &global_context,
        );
        let policy = Policy {
            valid_to: attributes.valid_to,
            created_at: attributes.created_at,
            policy_vec: attributes
                .alist
                .iter()
                .filter(|(tag, _type)| tag.0 <= 1)
                .map(|(tag, a_type)| (tag.clone(), a_type.clone()))
                .collect(),
            _phantom: Default::default(),
        };
        let prf_key = SecretKey::<ArCurve>::generate_non_zero(csprng);
        let aci = AccCredentialInfo {
            cred_holder_info: CredentialHolderInfo::<ArCurve> {
                id_cred: IdCredentials::generate(csprng),
            },
            prf_key,
        };
        let randomness = ps_sig::SigRetrievalRandomness::generate_non_zero(csprng);
        let id_use_data = IdObjectUseData { aci, randomness };
        let threshold = Threshold(1);
        let initial_account = InitialAccountData {
            keys: vec![(KeyIndex::from(0), KeyPair::generate(csprng))]
                .into_iter()
                .collect(),
            threshold: SignatureThreshold(1),
        };
        let (pio, _) =
            account_holder::generate_pio(&ip_context, threshold, &id_use_data, &initial_account)
                .expect("Generating the pre-identity object should succeed.");
        let expiry = TransactionTime::from_seconds((chrono::Utc::now().timestamp() + 3600) as u64);
        let (ip_signature, _) = identity_provider::verify_credentials(
            &pio,
            ip_context,
            &attributes,
            expiry,
            &id_provider_data.ip_secret_key,
            &id_provider_data.ip_cdi_secret_key,
        )
        .expect("signature on the credential must be valid...");
        let id_object = IdentityObject {
            pre_identity_object: pio,
            alist: attributes,
            signature: ip_signature,
        };
        let credential_data = CredentialData {
            keys: vec![(KeyIndex::from(0), KeyPair::generate(csprng))]
                .into_iter()
                .collect(),
            threshold: SignatureThreshold(1),
        };
        let (credential_deployment_info, _) = account_holder::create_credential(
            ip_context,
            &id_object,
            &id_use_data,
            0,
            policy,
            &credential_data,
            &SystemAttributeRandomness {},
            &Either::Left(expiry),
        )?;
        let address =
            account_address_from_registration_id(&credential_deployment_info.values.cred_id);
        let transaction = BlockItem::<Payload>::from(AccountCredentialMessage {
            message_expiry: expiry,
            credential: AccountCredential::Normal {
                cdi: credential_deployment_info,
            },
        });
        println!("let's send first account transaction for {}", address);
        let transaction_hash = node_client.send_block_item(&transaction).await?;
        println!("transaction_hash:{}", transaction_hash);
        let (block_hash, block_item_summary) =
            node_client.wait_until_finalized(&transaction_hash).await?;
        println!("Transaction finalized in block {}.", block_hash);
        println!("The outcome is {:#?}", block_item_summary);
        Ok(CreateAccountOutputs {
            address,
            keys: AccountKeys::from(credential_data),
        })
    }
}
