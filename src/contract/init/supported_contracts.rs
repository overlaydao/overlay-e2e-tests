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
    use concordium_base::common::Serial;
    use concordium_rust_sdk::smart_contracts::common::ContractAddress;
    use serde::Deserialize;

    #[derive(Serial, Deserialize)]
    pub struct UpdateContractStateParams {
        staking_contract_addr: ContractAddress,
        user_contract_addr: ContractAddress,
    }
}
