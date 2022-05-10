use super::{Page, Sort};
use crate::{Address, BlockHash, BlockNumber, TransactionHash};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr, TimestampSecondsWithFrac};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ERC20TokenTransfer {
    #[serde(deserialize_with = "crate::de_string_to_block_number")]
    pub block_number: BlockNumber,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    pub time_stamp: DateTime<Utc>,
    pub hash: TransactionHash,
    #[serde_as(as = "DisplayFromStr")]
    pub nonce: u64,
    pub block_hash: BlockHash,
    pub from: Address,
    pub contract_address: Address,
    pub to: Address,
    /// Value of the token transfer
    /// NOTE: Can be a very large amount, therefore currently a string
    pub value: String,
    pub token_name: String,
    pub token_symbol: String,
    #[serde_as(as = "DisplayFromStr")]
    pub token_decimal: u8,
    #[serde_as(as = "DisplayFromStr")]
    pub transaction_index: u64,
    #[serde(alias = "gas")]
    #[serde_as(as = "DisplayFromStr")]
    pub gas_limit: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub gas_price: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub gas_used: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub cumulative_gas_used: u128,
    pub input: String,
    #[serde_as(as = "DisplayFromStr")]
    pub confirmations: u128,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ERC721TokenTransfer {
    #[serde(deserialize_with = "crate::de_string_to_block_number")]
    pub block_number: BlockNumber,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    pub time_stamp: DateTime<Utc>,
    pub hash: TransactionHash,
    #[serde_as(as = "DisplayFromStr")]
    pub nonce: u64,
    pub block_hash: BlockHash,
    pub from: Address,
    pub contract_address: Address,
    pub to: Address,
    #[serde(alias = "tokenID")]
    pub token_id: String, // ENS token ids can be very large
    pub token_name: String,
    pub token_symbol: String,
    #[serde_as(as = "DisplayFromStr")]
    pub token_decimal: u8,
    #[serde_as(as = "DisplayFromStr")]
    pub transaction_index: u64,
    #[serde(alias = "gas")]
    #[serde_as(as = "DisplayFromStr")]
    pub gas_limit: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub gas_price: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub gas_used: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub cumulative_gas_used: u128,
    pub input: String,
    #[serde_as(as = "DisplayFromStr")]
    pub confirmations: u128,
}

#[derive(Default)]
pub struct TokenOptions<'a> {
    address: Option<&'a str>,
    contract_address: Option<&'a str>,
    /// * 'start_block' - An optional starting block number.
    start_block: Option<u64>,
    /// * 'end_block' - An optional end block number.
    end_block: Option<u64>,
    /// * 'page' - An optional page number and number of transactions returned.
    page: Option<Page>,
    /// * 'sort' - An optional sort order.
    sort: Option<Sort>,
}

impl<'a> TokenOptions<'a> {
    pub fn new(address: &'a str, contract_address: &'a str, start_block: u64, end_block: u64, page: Page, sort: Sort) -> TokenOptions<'a> {
        TokenOptions {
            address: Some(address),
            contract_address: Some(contract_address),
            start_block: Some(start_block),
            end_block: Some(end_block),
            page: Some(page),
            sort: Some(sort),
        }
    }

    pub fn new_block_range(start_block: u64, end_block: u64) -> TokenOptions<'a> {
        TokenOptions {
            start_block: Some(start_block),
            end_block: Some(end_block),
            ..Default::default()
        }
    }

    pub fn new_page(number: u8, offset: u16) -> TokenOptions<'a> {
        TokenOptions {
            page: Some(Page { number, offset }),
            ..Default::default()
        }
    }

    pub fn new_page_with_sort(number: u8, offset: u16, sort: Sort) -> TokenOptions<'a> {
        TokenOptions {
            page: Some(Page { number, offset }),
            sort: Some(sort),
            ..Default::default()
        }
    }

    pub fn address(&self) -> Option<&str> {
        self.address
    }

    pub fn contract_address(&self) -> Option<&str> {
        self.contract_address
    }

    pub fn start_block(&self) -> Option<u64> {
        self.start_block
    }

    pub fn end_block(&self) -> Option<u64> {
        self.end_block
    }

    pub fn page(&self) -> Option<&Page> {
        self.page.as_ref()
    }

    pub fn sort(&self) -> Option<&Sort> {
        self.sort.as_ref()
    }
}
