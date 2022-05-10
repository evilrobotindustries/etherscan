use crate::responses::ResponseStatus;
use crate::{APIError, Address, BlockHash, BlockNumber, RPCError, Result, TransactionHash, TypeExtensions, ACTION, ADDRESS, MODULE, URI};
use crate::{Tag, TAG};
use ethabi::ethereum_types::{U128, U64};
use serde::de::DeserializeOwned;
use serde::{
    de,
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_with::{serde_as, DisplayFromStr};
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
    pub async fn block_number(&self) -> Result<BlockNumber> {
        let parameters = &[(MODULE, PROXY), (ACTION, "eth_blockNumber")];
        self.get(parameters).await
    }

    /// Returns information about a block by block number
    ///
    /// # Arguments
    ///
    /// * 'block_number' - The block number
    pub async fn block(&self, block_number: &BlockNumber) -> Result<Block> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_getBlockByNumber"),
            ("tag", &TypeExtensions::format(block_number)),
            ("boolean", &false.to_string()),
        ];
        self.get(parameters).await
    }

    /// Returns the number of transactions in a block
    ///
    /// # Arguments
    ///
    /// * 'block_number' - The block number
    pub async fn block_transactions(&self, block_number: &BlockNumber) -> Result<u64> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_getBlockTransactionCountByNumber"),
            (TAG, &TypeExtensions::format(block_number)),
        ];
        self.get::<U64>(parameters).await.map(|t| t.as_u64())
    }

    /// Executes a new message call immediately without creating a transaction on the block chain
    ///
    /// # Arguments
    ///
    /// * 'contract_address' - The contract address to interact with
    /// * 'data' - The hash of the method signature and encoded parameters
    /// * 'tag' - The pre-defined block parameter, which defaults to latest if not provided.
    pub async fn call(&self, contract_address: &Address, data: &str, tag: Option<Tag>) -> Result<String> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_call"),
            ("to", &TypeExtensions::format(contract_address)),
            ("data", data),
            (TAG, tag.or(Some(Tag::Latest)).unwrap().to_string()),
        ];
        self.get(parameters).await
    }

    /// Returns code at a given address
    ///
    /// # Arguments
    ///
    /// * 'address' - The address to get code
    /// * 'tag' - The pre-defined block parameter
    pub async fn code(&self, address: &Address, tag: Option<Tag>) -> Result<String> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_getCode"),
            (ADDRESS, &TypeExtensions::format(address)),
            (TAG, tag.or(Some(Tag::Latest)).unwrap().to_string()),
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
    ///
    /// **Note:** The gas parameter is capped at 2x the current block gas limit.
    pub async fn estimate_gas(&self, contract_address: &Address, data: &str, value: u64, gas: u64, gas_price: u64) -> Result<u64> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_estimateGas"),
            ("to", &TypeExtensions::format(contract_address)),
            ("data", data),
            ("value", &TypeExtensions::format(&value)),
            ("gas", &TypeExtensions::format(&gas)),
            ("gasPrice", &TypeExtensions::format(&gas_price)),
        ];
        self.get::<U64>(parameters).await.map(|t| t.as_u64())
    }

    /// Returns the current price per gas in wei
    pub async fn gas_price(&self) -> Result<u64> {
        let parameters = &[(MODULE, PROXY), (ACTION, "eth_gasPrice")];
        self.get::<U64>(parameters).await.map(|t| t.as_u64())
    }

    /// Submits a pre-signed transaction for broadcast to the Ethereum network
    ///
    /// # Arguments
    ///
    /// * 'transaction' - the signed raw transaction data to broadcast
    pub async fn send_transaction(&self, transaction: String) -> Result<TransactionHash> {
        let parameters = &[(MODULE, PROXY), (ACTION, "eth_sendRawTransaction"), ("hex", &transaction)];
        self.get(parameters).await
    }

    /// Returns the value from a storage position at a given address.
    ///
    /// # Arguments
    ///
    /// * 'address' - The address to get code
    /// * 'position' - The position in storage
    /// * 'tag' - The pre-defined block parameter
    ///
    /// **Note:** This endpoint is still experimental and may have potential issues
    pub async fn storage_value(&self, address: &Address, position: u16, tag: Option<Tag>) -> Result<String> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_getStorageAt"),
            (ADDRESS, &TypeExtensions::format(address)),
            ("position", &TypeExtensions::format(&position)),
            (TAG, tag.or(Some(Tag::Latest)).unwrap().to_string()),
        ];
        self.get(parameters).await
    }

    /// Returns the information about a transaction requested by transaction hash
    ///
    /// # Arguments
    ///
    /// * 'hash' - The hash of the transaction
    pub async fn transaction(&self, hash: &TransactionHash) -> Result<Option<Transaction>> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_getTransactionByHash"),
            ("txhash", &TypeExtensions::format(hash)),
        ];
        self.get(parameters).await
    }

    /// Returns the receipt of a transaction by transaction hash
    ///
    /// # Arguments
    ///
    /// * 'hash' - The hash of the transaction
    pub async fn transaction_receipt(&self, hash: &TransactionHash) -> Result<Option<TransactionReceipt>> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_getTransactionReceipt"),
            ("txhash", &TypeExtensions::format(hash)),
        ];
        self.get(parameters).await
    }

    /// Returns information about a transaction by block number and transaction index position.
    ///
    /// # Arguments
    ///
    /// * 'block_number' - The block number
    /// * 'index' - the position of the transaction's index in the block
    pub async fn transaction_within_block(&self, block_number: BlockNumber, index: u16) -> Result<Option<Transaction>> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_getTransactionByBlockNumberAndIndex"),
            (TAG, &TypeExtensions::format(&block_number)),
            ("index", &TypeExtensions::format(&index)),
        ];
        self.get(parameters).await
    }

    /// Returns the number of outgoing transactions sent by an address
    ///
    /// # Arguments
    ///
    /// * 'address' - The address to get the number of transactions performed
    /// * 'tag' - The pre-defined block parameter, which defaults to latest if not provided.
    pub async fn transactions(&self, address: &Address, tag: Option<Tag>) -> Result<u64> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_getTransactionCount"),
            (ADDRESS, &TypeExtensions::format(address)),
            (TAG, tag.or(Some(Tag::Latest)).unwrap().to_string()),
        ];
        self.get::<U64>(parameters).await.map(|t| t.as_u64())
    }

    /// Returns information about a uncle by block number.
    ///
    /// # Arguments
    ///
    /// * 'block_number' - The block number
    /// * 'index' - the position of the uncle's index in the block
    pub async fn uncle(&self, block_number: BlockNumber, index: u16) -> Result<Block> {
        let parameters = &[
            (MODULE, PROXY),
            (ACTION, "eth_getUncleByBlockNumberAndIndex"),
            (TAG, &TypeExtensions::format(&block_number)),
            ("index", &TypeExtensions::format(&index)),
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
            .map_err(|e| APIError::from(e))
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub base_fee_per_gas: String,
    pub difficulty: String,
    pub extra_data: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub hash: BlockHash,
    pub logs_bloom: String,
    pub miner: String,
    pub mix_hash: String,
    pub nonce: String,
    #[serde_as(as = "DisplayFromStr")]
    pub number: BlockNumber,
    pub parent_hash: Option<BlockHash>,
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

struct Response<T> {
    #[allow(dead_code)]
    pub id: u32,
    #[allow(dead_code)]
    pub json_rpc: String,
    pub result: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// The hash of the block where this transaction was in (none if pending)
    pub block_hash: Option<BlockHash>,
    /// The block number where this transaction was in (none if pending)
    pub block_number: Option<BlockNumber>,
    /// Address of the sender
    pub from: Address,
    /// The gas provided by the sender,
    #[serde(deserialize_with = "de_hash_to_u64")]
    pub gas: u64,
    /// The gas price provided by the sender in Wei
    #[serde(deserialize_with = "de_hash_to_u64")]
    pub gas_price: u64,
    #[serde(deserialize_with = "de_hash_to_u64")]
    pub max_fee_per_gas: u64,
    #[serde(deserialize_with = "de_hash_to_u64")]
    pub max_priority_fee_per_gas: u64,
    /// The hash of the transaction
    pub hash: TransactionHash,
    /// The data sent along with the transaction.
    pub input: String,
    /// The number of transactions made by the sender prior to this one
    #[serde(deserialize_with = "de_hash_to_u64")]
    pub nonce: u64,
    /// Address of the receiver (none when its a contract creation transaction)
    pub to: Option<Address>,
    /// The transaction's index position in the block (none if pending)
    #[serde(deserialize_with = "de_hash_to_optional_u32")]
    pub transaction_index: Option<u32>,
    /// The value transferred in Wei
    #[serde(deserialize_with = "de_hash_to_u64")]
    pub value: u64,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "de_hash_to_u8")]
    pub transaction_type: u8,
    //pub access_list
    // The chain id of the transaction, if any.
    #[serde(deserialize_with = "de_hash_to_optional_u8")]
    pub chain_id: Option<u8>,
    /// The standardized V field of the signature
    pub v: String,
    /// The R field of the signature
    pub r: String,
    pub s: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    /// Hash of the block where this transaction was in
    pub block_hash: Option<BlockHash>,
    /// Block number where this transaction was added
    pub block_number: Option<BlockNumber>,
    /// The contract address created for contract creation, otherwise none
    pub contract_address: Option<Address>,
    /// The total gas used when this transaction was executed in the block
    #[serde(deserialize_with = "de_hash_to_u64")]
    pub cumulative_gas_used: u64,
    #[serde(deserialize_with = "de_hash_to_u64")]
    pub effective_gas_price: u64,
    /// Address of the sender
    pub from: Address,
    /// The amount of gas used by this specific transaction alone
    #[serde(deserialize_with = "de_hash_to_u64")]
    pub gas_used: u64,
    /// Array of log entries, which this transaction generated
    pub logs: Vec<LogEntry>,
    /// Bloom filter for light clients to quickly retrieve related logs
    pub logs_bloom: String,
    #[serde(deserialize_with = "de_hash_to_u8")]
    pub status: u8,
    /// Address of the receiver (none when its a contract creation transaction)
    pub to: Option<Address>,
    /// Hash of the transaction
    pub transaction_hash: TransactionHash,
    #[serde(deserialize_with = "de_hash_to_u16")]
    /// Integer of the transactions index position in the block
    pub transaction_index: u16,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "de_hash_to_u8")]
    pub transaction_type: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub address: Address,
    pub topics: Vec<String>,
    pub data: String,
    pub block_number: BlockNumber,
    pub transaction_hash: TransactionHash,
    #[serde(deserialize_with = "de_hash_to_u16")]
    pub transaction_index: u16,
    pub block_hash: BlockHash,
    #[serde(deserialize_with = "de_hash_to_u16")]
    pub log_index: u16,
    pub removed: bool,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Response<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        struct ResultVisitor<T>(PhantomData<fn() -> T>);

        const ID: &str = "id";
        const JSON_RPC: &str = "jsonrpc";
        const ERROR: &str = "error";
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
                        ERROR => {
                            let error = map.next_value::<RPCError>()?;
                            return Err(de::Error::custom(format!(
                                "rpc error:{}",
                                serde_json::to_string(&error).expect("could not serialize error")
                            )));
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

fn de_hash_to_u8<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<u8, D::Error> {
    U64::deserialize(deserializer).map(|v| v.as_u32() as u8)
}

fn de_hash_to_optional_u8<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<Option<u8>, D::Error> {
    U64::deserialize(deserializer).map(|v| Some(v.as_u32() as u8))
}

fn de_hash_to_u16<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<u16, D::Error> {
    U64::deserialize(deserializer).map(|v| v.as_u32() as u16)
}

fn de_hash_to_u32<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<u32, D::Error> {
    U64::deserialize(deserializer).map(|v| v.as_u32())
}

fn de_hash_to_optional_u32<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<Option<u32>, D::Error> {
    U64::deserialize(deserializer).map(|v| Some(v.as_u32()))
}

fn de_hash_to_u64<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<u64, D::Error> {
    U64::deserialize(deserializer).map(|v| v.as_u64())
}

fn de_hash_to_u128<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<u128, D::Error> {
    U128::deserialize(deserializer).map(|v| v.as_u128())
}
