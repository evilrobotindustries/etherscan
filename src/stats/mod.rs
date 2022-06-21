use super::Result;
use crate::{BlockNumber, Client, TypeExtensions, ACTION, MODULE};
use async_trait::async_trait;
use chrono::{Date, DateTime, NaiveDate, Utc};
use ethabi::Address;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_with::{serde_as, DisplayFromStr, TimestampSecondsWithFrac};

#[cfg(test)]
mod tests;

const STATS: &str = "stats";

#[async_trait]
pub trait Stats {
    /// Returns the size of the Ethereum blockchain, in bytes, over a date range
    ///
    /// # Arguments
    ///
    /// * 'start_date' - the start date
    /// * 'end_date' - the end date
    /// * 'client_type' - the Ethereum node client to use
    /// * 'sync_mode' - the type of node to run
    /// * 'sort' - the sorting preference
    async fn chain_size(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        client_type: ClientType,
        sync_mode: SyncMode,
        sort: Sort,
    ) -> Result<Vec<ChainSize>>;

    /// Returns the latest price of 1 ETH
    async fn last_price(&self) -> Result<Price>;

    /// Returns the total number of discoverable Ethereum nodes.
    async fn nodes(&self) -> Result<NodeStats>;

    /// Returns the current amount of an ERC-20 token in circulation.
    ///
    /// # Arguments
    ///
    /// * 'contract_address' - the contract address of the ERC-20 token
    async fn token_supply(&self, contract_address: &Address) -> Result<u128>;

    /// Returns the current amount of Ether in circulation excluding ETH2 Staking rewards and EIP1559 burnt fees
    async fn total_supply(&self) -> Result<u128>;

    /// Returns the current amount of Ether in circulation, ETH2 Staking rewards and EIP1559 burnt fees statistics.
    async fn total_supply_stats(&self) -> Result<TotalSupply>;
}

#[async_trait]
impl Stats for Client {
    async fn chain_size(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        client_type: ClientType,
        sync_mode: SyncMode,
        sort: Sort,
    ) -> Result<Vec<ChainSize>> {
        const DATE_FORMAT: &str = "%Y-%m-%d";
        let parameters = &[
            (MODULE, STATS),
            (ACTION, "chainsize"),
            ("startdate", &start_date.format(DATE_FORMAT).to_string()),
            ("enddate", &end_date.format(DATE_FORMAT).to_string()),
            ("clienttype", client_type.to_string()),
            ("syncmode", sync_mode.to_string()),
            ("sort", sort.to_string()),
        ];
        self.get(parameters).await
    }

    async fn last_price(&self) -> Result<Price> {
        let parameters = &[(MODULE, STATS), (ACTION, "ethprice")];
        self.get(parameters).await
    }

    async fn nodes(&self) -> Result<NodeStats> {
        let parameters = &[(MODULE, STATS), (ACTION, "nodecount")];
        self.get(parameters).await
    }

    async fn token_supply(&self, contract_address: &Address) -> Result<u128> {
        let parameters = &[
            (MODULE, STATS),
            (ACTION, "tokensupply"),
            ("contractaddress", &TypeExtensions::format(contract_address)),
        ];
        self.get::<String>(parameters).await.map(|v| v.parse::<u128>().unwrap_or(0))
    }

    async fn total_supply(&self) -> Result<u128> {
        let parameters = &[(MODULE, STATS), (ACTION, "ethsupply")];
        self.get::<String>(parameters).await.map(|v| v.parse::<u128>().unwrap_or(0))
    }

    /// Returns the current amount of Ether in circulation, ETH2 Staking rewards and EIP1559 burnt fees statistics.
    async fn total_supply_stats(&self) -> Result<TotalSupply> {
        let parameters = &[(MODULE, STATS), (ACTION, "ethsupply2")];
        self.get(parameters).await
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainSize {
    #[serde(deserialize_with = "crate::de_string_to_block_number")]
    pub block_number: BlockNumber,
    #[serde(rename = "chainTimeStamp")]
    #[serde(deserialize_with = "de_string_to_date")]
    pub date: Date<Utc>,
    #[serde(rename = "chainSize")]
    #[serde_as(as = "DisplayFromStr")]
    pub size: u64,
    #[serde(deserialize_with = "de_string_to_client_type")]
    pub client_type: ClientType,
    #[serde(deserialize_with = "de_string_to_sync_mode")]
    pub sync_mode: SyncMode,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeStats {
    #[serde(rename = "UTCDate")]
    #[serde(deserialize_with = "de_string_to_date")]
    pub date: Date<Utc>,
    #[serde(rename = "TotalNodeCount")]
    #[serde_as(as = "DisplayFromStr")]
    pub total_nodes: u64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Price {
    #[serde_as(as = "DisplayFromStr")]
    pub ethbtc: f32,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    pub ethbtc_timestamp: DateTime<Utc>,
    #[serde_as(as = "DisplayFromStr")]
    pub ethusd: f32,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    pub ethusd_timestamp: DateTime<Utc>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TotalSupply {
    #[serde_as(as = "DisplayFromStr")]
    pub eth_supply: u128,
    #[serde(rename = "Eth2Staking")]
    #[serde_as(as = "DisplayFromStr")]
    pub eth_staking: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub burnt_fees: u128,
}

fn de_string_to_date<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<Date<Utc>, D::Error> {
    let str_val = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&str_val, "%Y-%m-%d")
        .map(|d| Date::<Utc>::from_utc(d, Utc))
        .map_err(Error::custom)
}

fn de_string_to_client_type<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<ClientType, D::Error> {
    match String::deserialize(deserializer)?.as_str() {
        "Geth" => Ok(ClientType::GoEthereum),
        "Parity" => Ok(ClientType::Parity),
        other => Err(Error::custom(format!("could not match {other} to a client type"))),
    }
}

fn de_string_to_sync_mode<'a, D: Deserializer<'a>>(deserializer: D) -> std::result::Result<SyncMode, D::Error> {
    match String::deserialize(deserializer)?.as_str() {
        "Default" => Ok(SyncMode::Default),
        "Archive" => Ok(SyncMode::Archive),
        other => Err(Error::custom(format!("could not match {other} to a sync mode"))),
    }
}

#[derive(Debug, Deserialize)]
pub enum ClientType {
    GoEthereum,
    Parity,
}

impl ClientType {
    fn to_string(&self) -> &str {
        match &self {
            ClientType::GoEthereum => "geth",
            ClientType::Parity => "parity",
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum SyncMode {
    Default,
    Archive,
}

impl SyncMode {
    fn to_string(&self) -> &str {
        match &self {
            SyncMode::Default => "default",
            SyncMode::Archive => "archive",
        }
    }
}

pub enum Sort {
    Ascending,
    Descending,
}

impl Sort {
    fn to_string(&self) -> &str {
        match &self {
            Sort::Ascending => "asc",
            Sort::Descending => "desc",
        }
    }
}
