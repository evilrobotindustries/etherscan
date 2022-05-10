mod accounts;
mod contracts;
mod gas_tracker;
mod proxy;
mod responses;

use serde::{de, de::DeserializeOwned, Deserialize, Serialize};
use serde_with::DeserializeAs;
use std::error::Error;
use std::fmt::Debug;

const URI: &str = "https://api.etherscan.io/api";
const MODULE: &str = "module";
const ACTION: &str = "action";
const ADDRESS: &str = "address";
const TAG: &str = "tag";
const WEI_TO_ETH: f64 = 1_000_000_000_000_000_000f64;

type Result<T> = std::result::Result<T, crate::APIError>;
pub type Address = ethabi::Address;
pub type BlockHash = ethabi::ethereum_types::H256;
pub type BlockNumber = ethabi::ethereum_types::U64;
pub type TransactionHash = ethabi::ethereum_types::H256;

pub struct Client {
    api_key: String,
    client: reqwest::Client,
}

impl Client {
    fn new(api_key: impl Into<String>) -> Client {
        Client {
            api_key: api_key.into(),
            client: reqwest::Client::new(),
        }
    }

    async fn get<'de, T: DeserializeOwned>(&self, parameters: &[(&str, &str)]) -> Result<T> {
        self.client
            .get(URI)
            .query(&[("apikey", &self.api_key)])
            .query(parameters)
            .send()
            .await?
            .json::<responses::Response<T>>()
            .await
            .map(|r| r.result)
            .map_err(|e| APIError::from(e))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum APIError {
    #[error("Contract not verified")]
    ContractNotVerified,
    #[error("Deserialization Error")]
    DeserializationError { message: String },
    #[error("Invalid address")]
    InvalidAddress,
    #[error("Invalid API Key")]
    InvalidAPIKey { message: String },
    #[error("Rate Limit Reached")]
    RateLimitReached { message: String },
    #[error("RPC Error")]
    RPCError { code: i16, message: String },
    #[error("Too many addresses provided (max 20)")]
    TooManyAddresses,
    #[error("Request error")]
    TransportError {
        #[from]
        source: reqwest::Error,
    },
}

impl APIError {
    fn from(e: reqwest::Error) -> APIError {
        if e.is_decode() {
            if let Some(source) = e.source() {
                let source = source.downcast_ref::<serde_json::Error>().expect("serde_json error expected.");
                let source_message = source
                    .to_string()
                    .replace(&format!(" at line {} column {}", source.line(), source.column()), "");
                match source_message.as_str() {
                    "Max rate limit reached, please use API Key for higher rate limit" => {
                        return APIError::RateLimitReached { message: source_message }
                    }
                    "Max rate limit reached" => return APIError::RateLimitReached { message: source_message },
                    "Invalid API Key" => return APIError::InvalidAPIKey { message: source_message },
                    "Too many invalid api key attempts, please try again later" => {
                        return APIError::InvalidAPIKey { message: source_message }
                    }
                    "Contract source code not verified" => return APIError::ContractNotVerified,
                    "Invalid Address format" => return APIError::InvalidAddress,
                    _ => {}
                };
                if source_message.starts_with("rpc error:") {
                    if let Ok(error) = serde_json::from_str::<RPCError>(&source_message[10..]) {
                        return APIError::RPCError {
                            code: error.code,
                            message: error.message,
                        };
                    }
                }
                return APIError::DeserializationError {
                    message: source.to_string(),
                };
            }
        }
        APIError::TransportError { source: e }
    }
}

#[derive(Serialize, Deserialize)]
struct RPCError {
    code: i16,
    message: String,
}

fn wei_to_eth(value: u128) -> f64 {
    value as f64 / WEI_TO_ETH
}

struct BoolFromStr;

impl<'de> DeserializeAs<'de, bool> for BoolFromStr {
    fn deserialize_as<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<bool, D::Error> {
        let s = String::deserialize(deserializer).map_err(de::Error::custom)?;
        Ok(s == "1")
    }
}

struct WeiToEth;

impl<'de> DeserializeAs<'de, f64> for WeiToEth {
    fn deserialize_as<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<f64, D::Error> {
        let s = String::deserialize(deserializer).map_err(de::Error::custom)?;
        s.parse::<u128>().map(wei_to_eth).map_err(de::Error::custom)
    }
}

pub trait TypeExtensions {
    fn format(&self) -> String;
}

impl TypeExtensions for Address {
    fn format(&self) -> String {
        let address: ethabi::Address = self.0.into();
        format!("{:#x}", address)
    }
}

impl TypeExtensions for TransactionHash {
    fn format(&self) -> String {
        let hash: TransactionHash = self.0.into();
        format!("{:#x}", hash)
    }
}

impl TypeExtensions for BlockNumber {
    fn format(&self) -> String {
        let block_number = self.0[0];
        format!("{:#x}", block_number)
    }
}

impl TypeExtensions for u64 {
    fn format(&self) -> String {
        format!("{:#x}", self)
    }
}

impl TypeExtensions for u8 {
    fn format(&self) -> String {
        format!("{:#x}", self)
    }
}

impl TypeExtensions for u16 {
    fn format(&self) -> String {
        format!("{:#x}", self)
    }
}

pub enum Tag {
    Earliest,
    Pending,
    Latest,
}

impl Tag {
    fn to_string(&self) -> &'static str {
        match self {
            Tag::Latest => "latest",
            Tag::Earliest => "earliest",
            Tag::Pending => "pending",
        }
    }
}
