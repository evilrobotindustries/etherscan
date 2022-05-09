use super::{BoolFromStr, Result, ACTION, MODULE};
use crate::ADDRESS;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[cfg(test)]
mod tests;

const CONTRACT: &str = "contract";

pub struct Client {
    client: super::Client,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Client {
        Client {
            client: super::Client::new(api_key),
        }
    }

    /// Returns the Contract Application Binary Interface ( ABI ) of a verified smart contract.
    ///
    /// # Arguments
    ///
    /// * 'address' - A contract address that has verified source code
    pub async fn get_abi(&self, address: &str) -> Result<String> {
        let parameters = &[(MODULE, CONTRACT), (ACTION, "getabi"), (ADDRESS, address)];
        self.client.get(parameters).await
    }

    /// Returns the Solidity source code of a verified smart contract.
    ///
    /// # Arguments
    ///
    /// * 'address' - A contract address that has verified source code
    pub async fn get_source_code(&self, address: &str) -> Result<Vec<Contract>> {
        let parameters = &[(MODULE, CONTRACT), (ACTION, "getsourcecode"), (ADDRESS, address)];
        self.client.get(parameters).await
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Contract {
    pub source_code: String,
    #[serde(rename = "ABI")]
    pub abi: String,
    pub contract_name: String,
    pub compiler_version: String,
    #[serde_as(as = "BoolFromStr")]
    pub optimization_used: bool,
    #[serde_as(as = "DisplayFromStr")]
    pub runs: u64,
    pub constructor_arguments: String,
    #[serde(rename = "EVMVersion")]
    pub evm_version: String,
    pub library: String,
    pub license_type: String,
    #[serde_as(as = "BoolFromStr")]
    pub proxy: bool,
    pub implementation: String,
    pub swarm_source: String,
}
