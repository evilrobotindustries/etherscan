use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ERC20TokenTransfer {
    #[serde(alias = "blockNumber", deserialize_with = "super::super::de_u64_from_str")]
    pub block_number: u64,
    #[serde(alias = "timeStamp", deserialize_with = "super::super::de_u64_from_str")]
    pub time_stamp: u64,
    pub hash: String,
    #[serde(deserialize_with = "super::super::de_u64_from_str")]
    pub nonce: u64,
    #[serde(alias = "blockHash")]
    pub block_hash: String,
    pub from: String,
    #[serde(alias = "contractAddress")]
    pub contract_address: String,
    pub to: String,
    /// Value of the token transfer
    /// NOTE: Can be a very large amount, therefore currently a string
    pub value: String,
    #[serde(alias = "tokenName")]
    pub token_name: String,
    #[serde(alias = "tokenSymbol")]
    pub token_symbol: String,
    #[serde(alias = "tokenDecimal", deserialize_with = "super::super::de_u8_from_str")]
    pub token_decimal: u8,
    #[serde(alias = "transactionIndex", deserialize_with = "super::super::de_u64_from_str")]
    pub transaction_index: u64,
    #[serde(alias = "gas", deserialize_with = "super::super::de_u64_from_str")]
    pub gas_limit: u64,
    #[serde(alias = "gasPrice", deserialize_with = "super::super::de_u128_from_str")]
    pub gas_price: u128,
    #[serde(alias = "gasUsed", deserialize_with = "super::super::de_u128_from_str")]
    pub gas_used: u128,
    #[serde(alias = "cumulativeGasUsed", deserialize_with = "super::super::de_u128_from_str")]
    pub cumulative_gas_used: u128,
    pub input: String,
    #[serde(deserialize_with = "super::super::de_u128_from_str")]
    pub confirmations: u128,
}

#[derive(Debug, Deserialize)]
pub struct ERC721TokenTransfer {
    #[serde(alias = "blockNumber", deserialize_with = "super::super::de_u64_from_str")]
    pub block_number: u64,
    #[serde(alias = "timeStamp", deserialize_with = "super::super::de_u64_from_str")]
    pub time_stamp: u64,
    pub hash: String,
    #[serde(deserialize_with = "super::super::de_u64_from_str")]
    pub nonce: u64,
    #[serde(alias = "blockHash")]
    pub block_hash: String,
    pub from: String,
    #[serde(alias = "contractAddress")]
    pub contract_address: String,
    pub to: String,
    #[serde(alias = "tokenID")]
    pub token_id: String,
    #[serde(alias = "tokenName")]
    pub token_name: String,
    #[serde(alias = "tokenSymbol")]
    pub token_symbol: String,
    #[serde(alias = "tokenDecimal", deserialize_with = "super::super::de_u8_from_str")]
    pub token_decimal: u8,
    #[serde(alias = "transactionIndex", deserialize_with = "super::super::de_u64_from_str")]
    pub transaction_index: u64,
    #[serde(alias = "gas", deserialize_with = "super::super::de_u64_from_str")]
    pub gas_limit: u64,
    #[serde(alias = "gasPrice", deserialize_with = "super::super::de_u128_from_str")]
    pub gas_price: u128,
    #[serde(alias = "gasUsed", deserialize_with = "super::super::de_u128_from_str")]
    pub gas_used: u128,
    #[serde(alias = "cumulativeGasUsed", deserialize_with = "super::super::de_u128_from_str")]
    pub cumulative_gas_used: u128,
    pub input: String,
    #[serde(deserialize_with = "super::super::de_u128_from_str")]
    pub confirmations: u128,
}
