use crate::{Error, Result};
use std::{fmt, ops::Deref, str::FromStr};

const OVERLAY_USERS: &'static str = "overlay-users";
const OVERLAY_PROJECTS: &'static str = "overlay-projects";
const OVERLAY_SALES: &'static str = "pub_rido_ccd";
const OVERLAY_SALES_ALIAS: &'static str = "overlay-sales";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContractName {
    OverlayUsers,
    OverlayProjects,
    OverlaySales,
}

impl ContractName {
    pub fn as_str(&self) -> &str {
        match self {
            Self::OverlayUsers => OVERLAY_USERS,
            Self::OverlayProjects => OVERLAY_PROJECTS,
            Self::OverlaySales => OVERLAY_SALES,
        }
    }
}

impl AsRef<str> for ContractName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for ContractName {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ContractName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ContractName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            OVERLAY_USERS => Ok(Self::OverlayUsers),
            OVERLAY_PROJECTS => Ok(Self::OverlayProjects),
            OVERLAY_SALES => Ok(Self::OverlaySales),
            OVERLAY_SALES_ALIAS => Ok(Self::OverlaySales),
            _ => Err(Error::new_invalid_argument_error(
                "Unsupported contract name...",
            )),
        }
    }
}

pub mod overlay_projects {
    use concordium_std::{ContractAddress, Deserial, SchemaType, Serial};

    #[derive(Debug, Serial, Deserial, SchemaType)]
    pub struct UpdateContractStateParams {
        staking_contract_addr: ContractAddress,
        user_contract_addr: ContractAddress,
    }
}

pub mod overlay_sales {
    use concordium_std::{
        collections::BTreeMap, AccountAddress, Address, Duration, SchemaType, Serialize, Timestamp,
    };

    #[derive(Debug, Serialize, SchemaType)]
    pub struct InitParams {
        /// Account of the administrator of the entity running the IDO
        proj_admin: AccountAddress,
        /// Address of Overlay for receiving sale fee
        addr_ovl: Address,
        /// Address of Overlay for buy back burn
        addr_bbb: Address,
        /// IDO schedule(The process is split into some phases)
        open_at: BTreeMap<Timestamp, Prior>,
        /// Sale End Time
        close_at: Timestamp,
        /// User(sale particicants) can withdraw assets according to the vesting period
        vesting_period: BTreeMap<Duration, AllowedPercentage>,
        /// Swap price of the project token
        price_per_token: MicroCcd,
        /// Amount of project tokens contained in a unit
        token_per_unit: ContractTokenAmount,
        /// Hardcap
        max_units: UnitsAmount,
        /// Softcap
        min_units: UnitsAmount,
    }

    pub type AllowedPercentage = u8;
    pub type MicroCcd = u64;
    pub type ContractTokenAmount = concordium_cis2::TokenAmountU64;
    pub type UnitsAmount = u32;

    #[derive(Debug, Serialize, SchemaType)]
    pub enum Prior {
        TOP = 1,
        SECOND,
        ANY = 99,
    }
}
