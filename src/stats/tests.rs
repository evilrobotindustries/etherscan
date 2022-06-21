use super::Client;
use crate::stats::Stats;
use crate::stats::{ClientType, Sort, SyncMode};
use crate::{Address, BlockNumber};
use chrono::{Duration, NaiveDate, Utc};
use once_cell::sync::Lazy;
use std::str::FromStr;

const API_KEY: &str = "";

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new(API_KEY));

#[tokio::test]
async fn chain_size() -> Result<(), crate::APIError> {
    let start_date = NaiveDate::from_ymd(2019, 02, 01);
    let end_date = NaiveDate::from_ymd(2019, 02, 28);
    let chain_size = CLIENT
        .chain_size(start_date, end_date, ClientType::GoEthereum, SyncMode::Default, Sort::Ascending)
        .await?;
    assert_ne!(27, chain_size.len());
    for i in chain_size {
        assert_ne!(BlockNumber::zero(), i.block_number);
        assert!(i.date.naive_utc() >= start_date && i.date.naive_utc() <= end_date);
        assert!(i.size > 0);
        assert!(matches!(i.client_type, ClientType::GoEthereum));
        assert!(matches!(i.sync_mode, SyncMode::Default));
    }
    Ok(())
}

#[tokio::test]
async fn last_price() -> Result<(), crate::APIError> {
    let prices = CLIENT.last_price().await?;
    assert_ne!(0f32, prices.ethbtc);
    assert_eq!(prices.ethbtc_timestamp.date(), Utc::now().date());
    assert_ne!(0f32, prices.ethusd);
    assert_eq!(prices.ethusd_timestamp.date(), Utc::now().date());

    println!("{:#?}", prices);
    Ok(())
}

#[tokio::test]
async fn nodes() -> Result<(), crate::APIError> {
    let stats = CLIENT.nodes().await?;
    assert_eq!(Utc::now().date() - Duration::days(1), stats.date);
    assert_ne!(0, stats.total_nodes);
    println!("{:#?}", stats);
    Ok(())
}

#[tokio::test]
async fn token_supply() -> Result<(), crate::APIError> {
    let address = Address::from_str("0x57d90b64a1a57749b0f932f1a3395792e12e7055").expect("could not parse address");
    let supply = CLIENT.token_supply(&address).await?;
    assert_eq!(21265524714464, supply);
    Ok(())
}

#[tokio::test]
async fn total_supply() -> Result<(), crate::APIError> {
    let supply = CLIENT.total_supply().await?;
    assert_ne!(0, supply);
    Ok(())
}

#[tokio::test]
async fn total_supply_stats() -> Result<(), crate::APIError> {
    let stats = CLIENT.total_supply_stats().await?;
    assert_ne!(0, stats.eth_supply);
    assert_ne!(0, stats.eth_staking);
    assert_ne!(0, stats.burnt_fees);
    println!("{:#?}", stats);
    Ok(())
}
