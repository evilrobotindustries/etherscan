use super::Client;
use crate::blocks::Closest;
use crate::BlockNumber;
use chrono::{TimeZone, Utc};
use once_cell::sync::Lazy;

const API_KEY: &str = "";

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new(API_KEY));

#[tokio::test]
async fn at_time() -> Result<(), crate::APIError> {
    let time = Utc.timestamp(1578638524, 0);
    let block_number = CLIENT.at_time(time, Closest::Before).await?;
    assert_eq!(BlockNumber::from(9251482 as u64), block_number);
    Ok(())
}

#[tokio::test]
async fn estimated_time() -> Result<(), crate::APIError> {
    let block_number = BlockNumber::from(16701588 as u64);
    let estimated_time = CLIENT.estimated_time(&block_number).await?;
    assert_eq!(block_number, estimated_time.countdown_block);
    println!("Estimated time remaining until block {} is\n{:#?}", block_number, estimated_time);
    Ok(())
}

#[tokio::test]
async fn reward() -> Result<(), crate::APIError> {
    let block_number = BlockNumber::from(2165403 as u64);
    let block = CLIENT.reward(&block_number).await?;
    assert_eq!(block_number, block.block_number);
    println!("Reward of block {} is\n{:#?}", block_number, block);
    Ok(())
}
