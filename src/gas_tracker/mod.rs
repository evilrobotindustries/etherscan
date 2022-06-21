use super::{Result, ACTION, MODULE};
use crate::Client;
use async_trait::async_trait;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[cfg(test)]
mod tests;

const GAS_TRACKER: &str = "gastracker";

#[async_trait]
pub trait GasTracker {
    /// Returns the estimated time, in seconds, for a transaction to be confirmed on the blockchain
    ///
    /// # Arguments
    ///
    /// * 'gas_price' - the price paid per unit of gas, in wei
    async fn estimate_time(&self, gas_price: u64) -> Result<u64>;

    /// Returns the current Safe, Proposed and Fast gas prices
    async fn oracle(&self) -> Result<Oracle>;
}

#[async_trait]
impl GasTracker for Client {
    /// Returns the estimated time, in seconds, for a transaction to be confirmed on the blockchain
    ///
    /// # Arguments
    ///
    /// * 'gas_price' - the price paid per unit of gas, in wei
    async fn estimate_time(&self, gas_price: u64) -> Result<u64> {
        let parameters = &[(MODULE, GAS_TRACKER), (ACTION, "gasestimate"), ("gasprice", &gas_price.to_string())];
        let seconds = self.get::<String>(parameters).await?;
        Ok(seconds.parse::<u64>().unwrap_or(0))
    }

    /// Returns the current Safe, Proposed and Fast gas prices
    async fn oracle(&self) -> Result<Oracle> {
        let parameters = &[(MODULE, GAS_TRACKER), (ACTION, "gasoracle")];
        self.get(parameters).await
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Oracle {
    #[serde_as(as = "DisplayFromStr")]
    pub last_block: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub safe_gas_price: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub propose_gas_price: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub fast_gas_price: u32,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "suggestBaseFee")]
    pub suggest_base_fee: f32,
    #[serde(rename = "gasUsedRatio")]
    pub gas_used_ratio: String,
}
