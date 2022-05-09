use crate::responses::ResponseStatus;
use crate::{Result, ACTION, ADDRESS, MODULE, URI};
use serde::de::DeserializeOwned;
use serde::{
    de,
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;
use std::marker::PhantomData;

#[cfg(test)]
mod tests;

const PROXY: &str = "proxy";

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

    /// Returns the number of most recent block
    pub async fn block_number(&self) -> Result<String> {
        let parameters = &[(MODULE, PROXY), (ACTION, "eth_blockNumber")];
        self.get(parameters).await
    }

    /// Returns information about a block by block number
    ///
    /// # Arguments
    ///
    /// * 'block_number' - The block number, in hex
    pub async fn block(&self, block_number: &str) -> Result<Block> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_getBlockByNumber"),
            ("tag", block_number),
            ("boolean", &false.to_string()),
        ];
        self.get(parameters).await
    }

    /// Returns the number of transactions in a block
    ///
    /// # Arguments
    ///
    /// * 'block_number' - The block number, in hex
    pub async fn block_transactions(&self, block_number: &str) -> Result<String> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_getBlockTransactionCountByNumber"),
            ("tag", block_number),
        ];
        self.get(parameters).await
    }

    /// Executes a new message call immediately without creating a transaction on the block chain
    ///
    /// # Arguments
    ///
    /// * 'contract_address' - The contract address to interact with
    /// * 'data' - The hash of the method signature and encoded parameters
    pub async fn call(&self, contract_address: &str, data: &str) -> Result<String> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_call"),
            ("to", contract_address),
            ("data", data),
            ("tag", "latest"), // the pre-defined block parameter, either earliest, pending or latest
        ];
        self.get(parameters).await
    }

    /// Makes a call or transaction, which won't be added to the blockchain, but returns the used gas
    ///
    /// # Arguments
    ///
    /// * 'contract_address' - The contract address to interact with
    /// * 'data' - the hash of the method signature and encoded parameters
    /// * 'value' - the value sent in this transaction, in hex
    /// * 'gas' - the amount of gas provided for the transaction, in hex
    /// * 'gas_price' - the gas price paid for each unit of gas, in wei
    pub async fn estimate_gas(&self, contract_address: &str, data: &str, value: &str, gas: &str, gas_price: &str) -> Result<String> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_estimateGas"),
            ("to", contract_address),
            ("data", data),
            ("value", value),
            ("gas", gas),
            ("gasPrice", gas_price),
        ];
        self.get(parameters).await
    }

    /// Returns the current price per gas in wei
    pub async fn gas_price(&self) -> Result<String> {
        let parameters = &[(MODULE, PROXY), (ACTION, "eth_gasPrice")];
        self.get(parameters).await
    }

    /// Returns the number of transactions performed by an address
    ///
    /// # Arguments
    ///
    /// * 'address' - The address to get the number of transactions performed
    pub async fn transactions(&self, address: &str) -> Result<String> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_getTransactionCount"),
            (ADDRESS, address),
            ("tag", "latest"), // the string pre-defined block parameter, either earliest, pending or latest
        ];
        self.get(parameters).await
    }

    /// Returns information about a uncle by block number.
    ///
    /// # Arguments
    ///
    /// * 'block_number' - The block number, in hex
    /// * 'index' - the position of the uncle's index in the block, in hex
    pub async fn uncle(&self, block_number: &str, index: &str) -> Result<Block> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_getUncleByBlockNumberAndIndex"),
            ("tag", block_number),
            ("index", index),
        ];
        self.get(parameters).await
    }

    async fn get<'de, T: DeserializeOwned>(&self, parameters: &[(&str, &str)]) -> Result<T> {
        self.client
            .get(URI)
            .query(&[("apikey", &self.api_key)])
            .query(parameters)
            .send()
            .await?
            .json::<Response<T>>()
            .await
            .map(|r| r.result)
            .map_err(|e| crate::map_error(e))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub base_fee_per_gas: String,
    pub difficulty: String,
    pub extra_data: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub hash: String,
    pub logs_bloom: String,
    pub miner: String,
    pub mix_hash: String,
    pub nonce: String,
    pub number: String,
    pub parent_hash: String,
    pub receipts_root: String,
    pub sha3_uncles: String,
    pub size: String,
    pub state_root: String,
    pub timestamp: String,
    pub total_difficulty: Option<String>,
    pub transactions: Option<Vec<String>>,
    pub transactions_root: String,
    pub uncles: Vec<String>,
}

#[derive(Debug)]
struct Response<T> {
    pub id: u32,
    pub json_rpc: String,
    pub result: T,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Response<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        struct ResultVisitor<T>(PhantomData<fn() -> T>);

        const ID: &str = "id";
        const JSON_RPC: &str = "jsonrpc";
        const STATUS: &str = "status";
        const MESSAGE: &str = "message";
        const RESULT: &str = "result";

        impl<'de, T: Deserialize<'de>> Visitor<'de> for ResultVisitor<T> {
            type Value = Response<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Response<T>")
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> std::result::Result<Response<T>, V::Error> {
                let mut id = None;
                let mut json_rpc = None;
                let mut result = None;
                let mut status = None;
                let mut message: Option<String> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        ID => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field(ID));
                            }
                            id = Some(map.next_value::<u32>()?);
                        }
                        JSON_RPC => {
                            if json_rpc.is_some() {
                                return Err(de::Error::duplicate_field(JSON_RPC));
                            }
                            json_rpc = Some(map.next_value::<String>()?);
                        }
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
                        _ => {
                            // Ignore value
                            let _ = map.next_value::<()>();
                        }
                    }
                }
                let id = id.ok_or_else(|| de::Error::missing_field(ID))?;
                let json_rpc = json_rpc.ok_or_else(|| de::Error::missing_field(JSON_RPC))?;
                let result = result.ok_or_else(|| de::Error::missing_field(RESULT))?;
                Ok(Response::<T> { id, json_rpc, result })
            }
        }

        const FIELDS: &[&str] = &[ID, JSON_RPC, STATUS, MESSAGE, RESULT];
        deserializer.deserialize_struct("Response", FIELDS, ResultVisitor(PhantomData))
    }
}
