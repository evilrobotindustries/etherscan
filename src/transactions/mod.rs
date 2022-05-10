use super::BoolFromStr;
use super::Result;
use crate::{TransactionHash, TypeExtensions, ACTION, MODULE};
use serde::Deserialize;
use serde_with::serde_as;

#[cfg(test)]
mod tests;

const TRANSACTION: &str = "transaction";

pub struct Client {
    client: super::Client,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Client {
        Client {
            client: super::Client::new(api_key),
        }
    }

    pub fn from(client: super::Client) -> Client {
        Client { client }
    }

    /// Returns the status code of a contract execution
    ///
    /// # Arguments
    ///
    /// * 'hash' - The transaction hash to check the execution status
    pub async fn execution_status(&self, hash: &TransactionHash) -> Result<ExecutionStatus> {
        let parameters = &[
            (MODULE, TRANSACTION),
            (ACTION, "getstatus"),
            ("txhash", &TypeExtensions::format(hash)),
        ];
        self.client.get(parameters).await
    }

    /// Returns the status code of a contract execution
    ///
    /// # Arguments
    ///
    /// * 'hash' - The transaction hash to check the receipt status
    /// * 'returns' -  false for failed transactions and true for successful transactions.
    ///
    /// **Note:** Only applicable for post Byzantium Fork transactions
    pub async fn receipt_status(&self, hash: &TransactionHash) -> Result<bool> {
        let parameters = &[
            (MODULE, TRANSACTION),
            (ACTION, "gettxreceiptstatus"),
            ("txhash", &TypeExtensions::format(hash)),
        ];
        Ok(self.client.get::<TransactionReceiptStatus>(parameters).await?.status)
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
