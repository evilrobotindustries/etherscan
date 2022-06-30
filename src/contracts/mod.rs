use super::{BoolFromStr, Result, ACTION, MODULE};
use crate::{APIError, Client, TypeExtensions, ADDRESS};
use async_trait::async_trait;
use ethabi::Address;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::str;

#[cfg(test)]
mod tests;

const CONTRACT: &str = "contract";

pub type ABI = ethabi::Contract;
pub type Function = ethabi::Function;
pub type Token = ethabi::token::Token;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Contracts {
    /// Returns the Contract Application Binary Interface ( ABI ) of a verified smart contract.
    ///
    /// # Arguments
    ///
    /// * 'address' - A contract address that has verified source code
    async fn get_abi(&self, address: &Address) -> Result<ABI>;

    /// Returns the Solidity source code of a verified smart contract.
    ///
    /// # Arguments
    ///
    /// * 'address' - A contract address that has verified source code
    async fn get_source_code(&self, address: &Address) -> Result<Vec<Contract>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Contracts for Client {
    async fn get_abi(&self, address: &Address) -> Result<ABI> {
        let parameters = &[(MODULE, CONTRACT), (ACTION, "getabi"), (ADDRESS, &TypeExtensions::format(address))];
        let abi: String = self.get(parameters).await?;
        ABI::load(abi.as_bytes()).map_err(|e| APIError::DeserializationError { message: e.to_string() })
    }

    async fn get_source_code(&self, address: &Address) -> Result<Vec<Contract>> {
        let parameters = &[
            (MODULE, CONTRACT),
            (ACTION, "getsourcecode"),
            (ADDRESS, &TypeExtensions::format(address)),
        ];
        self.get(parameters).await
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Contract {
    pub source_code: String,
    #[serde(rename = "ABI")]
    #[serde(deserialize_with = "de_string_to_abi")]
    pub abi: ABI,
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

fn de_string_to_abi<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<ABI, D::Error> {
    let str_val = String::deserialize(deserializer)?;
    ABI::load(str_val.as_bytes()).map_err(D::Error::custom)
}
