use super::Result;
use crate::{APIError, BlockNumber, Client, ACTION, MODULE};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use ethabi::Address;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr, TimestampSecondsWithFrac};
use std::str::FromStr;

#[cfg(test)]
mod tests;

const BLOCK: &str = "block";

#[async_trait]
pub trait Blocks {
    /// Returns the block number that was mined at a certain timestamp
    ///
    /// # Arguments
    ///
    /// * 'timestamp' - the integer representing the Unix timestamp in seconds.
    /// * 'closest' - the closest available block to the provided timestamp, either before or after
    async fn at_time(&self, time: DateTime<Utc>, closest: Closest) -> Result<BlockNumber>;

    /// Returns the estimated time remaining until a certain block is mined.
    ///
    /// # Arguments
    ///
    /// * 'block_number' - the integer block number to estimate time remaining to be mined
    async fn estimated_time(&self, block_number: &BlockNumber) -> Result<EstimatedTime>;

    /// Returns the block reward and 'uncle' block rewards
    ///
    /// # Arguments
    ///
    /// * 'block_number' - the integer block number to check block rewards
    async fn reward(&self, block_number: &BlockNumber) -> Result<Block>;
}

#[async_trait]
impl Blocks for Client {
    async fn at_time(&self, time: DateTime<Utc>, closest: Closest) -> Result<BlockNumber> {
        let parameters = &[
            (MODULE, BLOCK),
            (ACTION, "getblocknobytime"),
            ("timestamp", &time.timestamp().to_string()),
            ("closest", closest.to_string()),
        ];
        let value = self.get::<String>(parameters).await?;
        u64::from_str(&value)
            .map(|v| BlockNumber::from(v))
            .map_err(|_| APIError::DeserializationError {
                message: "unable to deserialize result as block number".to_string(),
            })
    }

    async fn estimated_time(&self, block_number: &BlockNumber) -> Result<EstimatedTime> {
        let parameters = &[
            (MODULE, BLOCK),
            (ACTION, "getblockcountdown"),
            ("blockno", &block_number.to_string()),
        ];
        self.get(parameters).await
    }

    async fn reward(&self, block_number: &BlockNumber) -> Result<Block> {
        let parameters = &[(MODULE, BLOCK), (ACTION, "getblockreward"), ("blockno", &block_number.to_string())];
        self.get(parameters).await
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(deserialize_with = "crate::de_string_to_block_number")]
    pub block_number: BlockNumber,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    pub time_stamp: DateTime<Utc>,
    pub block_miner: Address,
    #[serde_as(as = "DisplayFromStr")]
    pub block_reward: u128,
    pub uncles: Vec<Uncle>,
    #[serde_as(as = "DisplayFromStr")]
    pub uncle_inclusion_reward: u128,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EstimatedTime {
    #[serde(deserialize_with = "crate::de_string_to_block_number")]
    pub current_block: BlockNumber,
    #[serde(deserialize_with = "crate::de_string_to_block_number")]
    pub countdown_block: BlockNumber,
    #[serde(deserialize_with = "crate::de_string_to_block_number")]
    pub remaining_block: BlockNumber,
    #[serde_as(as = "DisplayFromStr")]
    pub estimate_time_in_sec: f32,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uncle {
    pub miner: Address,
    #[serde_as(as = "DisplayFromStr")]
    pub uncle_position: u16,
    #[serde(rename = "blockreward")]
    #[serde_as(as = "DisplayFromStr")]
    pub block_reward: u128,
}

pub enum Closest {
    Before,
    After,
}

impl Closest {
    fn to_string(&self) -> &str {
        match self {
            Closest::Before => "before",
            Closest::After => "after",
        }
    }
}
