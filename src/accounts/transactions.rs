use super::{Page, Sort};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(alias = "blockNumber", deserialize_with = "super::super::de_u64_from_str")]
    pub block_number: u64,
    #[serde(alias = "timeStamp", deserialize_with = "super::super::de_u64_from_str")]
    pub time_stamp: u64,
    pub hash: String,
    #[serde(deserialize_with = "super::super::de_u64_from_str")]
    pub nonce: u64,
    #[serde(alias = "blockHash")]
    pub block_hash: String,
    #[serde(alias = "transactionIndex", deserialize_with = "super::super::de_u64_from_str")]
    pub transaction_index: u64,
    pub from: String,
    pub to: String,
    #[serde(deserialize_with = "super::super::de_u128_from_str")]
    pub value: u128,
    #[serde(alias = "gas", deserialize_with = "super::super::de_u64_from_str")]
    pub gas_limit: u64,
    #[serde(alias = "gasPrice", deserialize_with = "super::super::de_u128_from_str")]
    pub gas_price: u128,
    #[serde(alias = "isError", deserialize_with = "super::super::de_bool_from_str")]
    pub is_error: bool,
    #[serde(alias = "txreceipt_status")]
    pub txreceipt_status: String,
    pub input: String,
    #[serde(alias = "contractAddress")]
    pub contract_address: String,
    #[serde(alias = "cumulativeGasUsed", deserialize_with = "super::super::de_u128_from_str")]
    pub cumulative_gas_used: u128,
    #[serde(alias = "gasUsed", deserialize_with = "super::super::de_u128_from_str")]
    pub gas_used: u128,
    #[serde(deserialize_with = "super::super::de_u128_from_str")]
    pub confirmations: u128,
}

#[derive(Debug, Deserialize)]
pub struct InternalTransaction {
    #[serde(alias = "blockNumber", deserialize_with = "super::super::de_u64_from_str")]
    pub block_number: u64,
    #[serde(alias = "timeStamp", deserialize_with = "super::super::de_u64_from_str")]
    pub time_stamp: u64,
    pub hash: Option<String>,
    pub from: String,
    pub to: String,
    #[serde(deserialize_with = "super::super::de_u128_from_str")]
    pub value: u128,
    #[serde(alias = "contractAddress")]
    pub contract_address: String,
    pub input: String,
    #[serde(alias = "type")]
    pub transaction_type: String,
    #[serde(alias = "gas", deserialize_with = "super::super::de_u64_from_str")]
    pub gas: u64,
    #[serde(alias = "gasUsed", deserialize_with = "super::super::de_u128_from_str")]
    pub gas_used: u128,
    #[serde(alias = "traceId")]
    pub trace_id: Option<String>,
    /// Returns false for successful transactions and true for rejected/cancelled transactions.
    #[serde(alias = "isError", deserialize_with = "super::super::de_bool_from_str")]
    pub is_error: bool,
    #[serde(alias = "errCode")]
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
