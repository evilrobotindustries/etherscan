use super::BoolFromStr;
use super::Result;
use crate::{Client, TransactionHash, TypeExtensions, ACTION, MODULE};
use async_trait::async_trait;
use serde::Deserialize;
use serde_with::serde_as;

#[cfg(test)]
mod tests;

const TRANSACTION: &str = "transaction";

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Transactions {
    /// Returns the status code of a contract execution
    ///
    /// # Arguments
    ///
    /// * 'hash' - The transaction hash to check the execution status
    async fn execution_status(&self, hash: &TransactionHash) -> Result<ExecutionStatus>;

    /// Returns the status code of a contract execution
    ///
    /// # Arguments
    ///
    /// * 'hash' - The transaction hash to check the receipt status
    /// * 'returns' -  false for failed transactions and true for successful transactions.
    ///
    /// **Note:** Only applicable for post Byzantium Fork transactions
    async fn receipt_status(&self, hash: &TransactionHash) -> Result<bool>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Transactions for Client {
    async fn execution_status(&self, hash: &TransactionHash) -> Result<ExecutionStatus> {
        let parameters = &[
            (MODULE, TRANSACTION),
            (ACTION, "getstatus"),
            ("txhash", &TypeExtensions::format(hash)),
        ];
        self.get(parameters).await
    }

    async fn receipt_status(&self, hash: &TransactionHash) -> Result<bool> {
        let parameters = &[
            (MODULE, TRANSACTION),
            (ACTION, "gettxreceiptstatus"),
            ("txhash", &TypeExtensions::format(hash)),
        ];
        Ok(self.get::<TransactionReceiptStatus>(parameters).await?.status)
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionStatus {
    #[serde_as(as = "BoolFromStr")]
    pub is_error: bool,
    #[serde(rename = "errDescription")]
    pub error_description: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransactionReceiptStatus {
    #[serde_as(as = "BoolFromStr")]
    status: bool,
}
