use super::{BoolFromStr, Result, ACTION, MODULE};
use crate::{APIError, ADDRESS};
use serde::de::{value, IntoDeserializer};
use serde::{Deserialize, Deserializer};
use serde_with::{serde_as, DisplayFromStr};
use std::str;
use std::str::FromStr;

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
    pub async fn get_abi(&self, address: &str) -> Result<Vec<Descriptor>> {
        let parameters = &[(MODULE, CONTRACT), (ACTION, "getabi"), (ADDRESS, address)];
        let abi: String = self.client.get(parameters).await?;
        serde_json::from_str(&abi).map_err(|e| APIError::DeserializationError { message: e.to_string() })
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

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Descriptor {
    Constructor {
        inputs: Vec<Input>,
    },
    Function {
        name: String,
        inputs: Vec<Input>,
        outputs: Vec<Output>,
        constant: Option<bool>,
        state_mutability: Option<StateMutability>,
    },
    Receive,
    Fallback {
        state_mutability: StateMutability,
    },
    Event {
        name: String,
        inputs: Vec<Input>,
        anonymous: bool,
    },
    Error {
        name: String,
        inputs: Vec<Input>,
    },
}

#[derive(Deserialize, Debug)]
pub struct Input {
    name: String,
    #[serde(rename = "type")]
    input_type: Type,
    components: Option<String>,
    indexed: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Output {
    name: String,
    #[serde(rename = "type")]
    output_type: Type,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    // Does not read blockchain state
    Pure,
    // Does not modify blockchain state
    View,
    // Does not accept Ether
    NonPayable,
    // Accepts Ether
    Payable,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(remote = "Type")]
pub enum Type {
    Address,
    #[serde(rename = "bool")]
    Boolean,
    Bytes,
    Bytes32,
    Fixed,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Int256,
    UFixed,
    UInt,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt126,
    UInt256,
    String,
    Other(String),
}

impl FromStr for Type {
    type Err = value::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or_else(|_| Self::Other(s));
        Ok(deserialized)
    }
}
