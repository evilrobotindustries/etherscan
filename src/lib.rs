mod accounts;
mod contracts;
mod gas_tracker;
mod proxy;
mod responses;

use serde::{de, de::DeserializeOwned, Deserialize, Deserializer};
use serde_with::DeserializeAs;
use std::error::Error;

const URI: &str = "https://api.etherscan.io/api";
const MODULE: &str = "module";
const ACTION: &str = "action";
const ADDRESS: &str = "address";
const WEI_TO_ETH: f64 = 1_000_000_000_000_000_000f64;

type Result<T> = std::result::Result<T, crate::APIError>;

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
            .map_err(|e| map_error(e))
    }
}

fn map_error(e: reqwest::Error) -> APIError {
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
                "Too many invalid api key attempts, please try again later" => return APIError::InvalidAPIKey { message: source_message },
                "Contract source code not verified" => return APIError::ContractNotVerified,
                "Invalid Address format" => return APIError::InvalidAddress,
                _ => {}
            };
            return APIError::DeserializationError {
                message: source.to_string(),
            };
        }
    }
    APIError::TransportError { source: e }
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
    #[error("Too many addresses provided (max 20)")]
    TooManyAddresses,
    #[error("Request error")]
    TransportError {
        #[from]
        source: reqwest::Error,
    },
}

fn de_wei_to_eth<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<f64, D::Error> {
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<u128>().map(wei_to_eth).map_err(de::Error::custom)
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
