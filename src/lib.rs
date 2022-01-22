mod accounts;

use serde::{
    de,
    de::{DeserializeOwned},
    Deserialize, Deserializer,
};
use std::error::Error;

const URI: &str = "https://api.etherscan.io/api";
const MODULE: &str = "module";
const ACTION: &str = "action";
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
            .map_err(|e: reqwest::Error| {
                if e.is_decode() {
                    if let Some(source) = e.source() {
                        let source = source
                            .downcast_ref::<serde_json::Error>()
                            .expect("serde_json error expected.");
                        let source = source
                            .to_string()
                            .replace(&format!(" at line {} column {}", source.line(), source.column()), "");
                        match source.as_str() {
                            "Max rate limit reached, please use API Key for higher rate limit" => {
                                return APIError::RateLimitReached { message: source }
                            }
                            "Max rate limit reached" => return APIError::RateLimitReached { message: source },
                            "Invalid API Key" => return APIError::InvalidAPIKey { message: source },
                            "Too many invalid api key attempts, please try again later" => {
                                return APIError::InvalidAPIKey { message: source }
                            }
                            _ => {}
                        };
                    }
                }
                APIError::TransportError { source: e }
            })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum APIError {
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

fn de_u8_from_str<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<u8, D::Error> {
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<u8>().map_err(de::Error::custom)
}

fn de_f64_from_str<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<f64, D::Error> {
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f64>().map_err(de::Error::custom)
}

fn de_u64_from_str<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<u64, D::Error> {
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<u64>().map_err(de::Error::custom)
}

fn de_u128_from_str<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<u128, D::Error> {
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<u128>().map_err(de::Error::custom)
}

fn de_bool_from_str<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<bool, D::Error> {
    let str_val = String::deserialize(deserializer)?;
    Ok(str_val == "1")
}

fn de_wei_to_eth<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<f64, D::Error> {
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<u128>().map(wei_to_eth).map_err(de::Error::custom)
}

fn wei_to_eth(value: u128) -> f64 {
    value as f64 / WEI_TO_ETH
}





mod responses {
    use std::fmt;
    use std::marker::PhantomData;
    use serde::{
        de,
        de::{MapAccess, Visitor},
        Deserialize, Deserializer,
    };

    #[derive(Debug)]
    pub struct Response<T> {
        pub status: ResponseStatus,
        pub message: String,
        pub result: T,
    }

    impl<T> Response<T> {
        pub fn new(status: ResponseStatus, message: String, result: T) -> Response<T> {
            Response { status, message, result }
        }
    }

    #[derive(Debug, Deserialize)]
    pub enum ResponseStatus {
        Success = 1,
        Failed = 0,
    }

    impl<'de, T: Deserialize<'de>> Deserialize<'de> for Response<T> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
            struct ResultVisitor<T>(PhantomData<fn() -> T>);

            impl<'de, T: Deserialize<'de>> Visitor<'de> for ResultVisitor<T> {
                type Value = Response<T>;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("struct Response<T>")
                }

                fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> std::result::Result<Response<T>, V::Error> {
                    const STATUS: &str = "status";
                    const MESSAGE: &str = "message";
                    const RESULT: &str = "result";

                    let mut status = None;
                    let mut message = None;
                    let mut result = None;
                    while let Some(key) = map.next_key()? {
                        match key {
                            STATUS => {
                                if status.is_some() {
                                    return Err(de::Error::duplicate_field(STATUS));
                                }
                                let value: String = map.next_value()?;
                                status = Some(match value.as_str() {
                                    "1" => ResponseStatus::Success,
                                    "0" => ResponseStatus::Failed,
                                    _ => panic!("Unknown value: {}", value),
                                });
                            }
                            MESSAGE => {
                                if message.is_some() {
                                    return Err(de::Error::duplicate_field(MESSAGE));
                                }
                                message = Some(map.next_value()?);

                                if let Some(ResponseStatus::Failed) = status {
                                    if let Some(message) = { &message } {
                                        if message == "NOTOK" {
                                            // Get message from next value
                                            let next: Option<(String, String)> = map.next_entry()?;
                                            if let Some((_, value)) = next {
                                                return Err(de::Error::custom(value));
                                            }
                                        }
                                    }
                                }
                            }
                            RESULT => {
                                if result.is_some() {
                                    return Err(de::Error::duplicate_field(RESULT));
                                }
                                if status.is_none() {
                                    return Err(de::Error::custom("status not deserialised yet"));
                                }

                                // Return result message as error if failed
                                if let Some(ResponseStatus::Failed) = status {
                                    if let Some(message) = { &message } {
                                        // Exclude empty result, which returns as status failed
                                        if message != "No transactions found" {
                                            let value: String = map.next_value()?;
                                            return Err(de::Error::custom(value));
                                        }
                                    }
                                }

                                result = Some(map.next_value()?);
                            }
                            _ => {}
                        }
                    }
                    let status = status.ok_or_else(|| de::Error::missing_field("status"))?;
                    let message = message.ok_or_else(|| de::Error::missing_field("message"))?;
                    let result = result.ok_or_else(|| de::Error::missing_field("result"))?;
                    Ok(Response::<T>::new(status, message, result))
                }
            }

            const FIELDS: &[&str] = &["status", "message", "result"];
            deserializer.deserialize_struct("Duration", FIELDS, ResultVisitor(PhantomData))
        }
    }
}