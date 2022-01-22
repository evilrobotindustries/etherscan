use super::{super::BoolFromStr, Page, Sort};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr, TimestampSecondsWithFrac};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde_as(as = "DisplayFromStr")]
    pub block_number: u64,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    pub time_stamp: DateTime<Utc>,
    pub hash: String,
    #[serde_as(as = "DisplayFromStr")]
    pub nonce: u64,
    pub block_hash: String,
    #[serde_as(as = "DisplayFromStr")]
    pub transaction_index: u64,
    pub from: String,
    pub to: String,
    #[serde_as(as = "DisplayFromStr")]
    pub value: u128,
    #[serde(alias = "gas")]
    #[serde_as(as = "DisplayFromStr")]
    pub gas_limit: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub gas_price: u128,
    #[serde_as(as = "BoolFromStr")]
    pub is_error: bool,
    #[serde(alias = "txreceipt_status")]
    pub tx_receipt_status: String,
    pub input: String,
    pub contract_address: String,
    #[serde_as(as = "DisplayFromStr")]
    pub cumulative_gas_used: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub gas_used: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub confirmations: u128,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransaction {
    #[serde_as(as = "DisplayFromStr")]
    pub block_number: u64,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    pub time_stamp: DateTime<Utc>,
    pub hash: Option<String>,
    pub from: String,
    pub to: String,
    #[serde_as(as = "DisplayFromStr")]
    pub value: u128,
    pub contract_address: String,
    pub input: String,
    #[serde(alias = "type")]
    pub transaction_type: String,
    #[serde_as(as = "DisplayFromStr")]
    pub gas: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub gas_used: u128,
    pub trace_id: Option<String>,
    #[serde_as(as = "BoolFromStr")]
    pub is_error: bool,
    pub err_code: String,
}

#[derive(Default)]
pub struct TransactionOptions {
    /// * 'start_block' - An optional starting block number.
    start_block: Option<u64>,
    /// * 'end_block' - An optional end block number.
    end_block: Option<u64>,
    /// * 'page' - An optional page number and number of transactions returned.
    page: Option<Page>,
    /// * 'sort' - An optional sort order.
    sort: Option<Sort>,
}

impl TransactionOptions {
    pub fn new(start_block: u64, end_block: u64, page: Page, sort: Sort) -> TransactionOptions {
        TransactionOptions {
            start_block: Some(start_block),
            end_block: Some(end_block),
            page: Some(page),
            sort: Some(sort),
        }
    }

    pub fn new_block_range(start_block: u64, end_block: u64) -> TransactionOptions {
        TransactionOptions {
            start_block: Some(start_block),
            end_block: Some(end_block),
            ..Default::default()
        }
    }

    pub fn new_page(number: u8, offset: u16) -> TransactionOptions {
        TransactionOptions {
            page: Some(Page { number, offset }),
            ..Default::default()
        }
    }

    pub fn new_page_with_sort(number: u8, offset: u16, sort: Sort) -> TransactionOptions {
        TransactionOptions {
            page: Some(Page { number, offset }),
            sort: Some(sort),
            ..Default::default()
        }
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
