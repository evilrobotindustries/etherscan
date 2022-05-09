use super::Client;
use crate::{Address, BlockNumber, Tag};
use ethabi::ethereum_types::U64;
use once_cell::sync::Lazy;
use std::str::FromStr;

const API_KEY: &str = "";
const BLOCK_NUMBER: &str = "0xc63251";

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new(API_KEY));

#[tokio::test]
async fn block_number() -> Result<(), crate::APIError> {
    let block_number = CLIENT.block_number().await?;
    assert!(block_number > BlockNumber::zero());
    println!("Most recent block is {}", block_number);
    Ok(())
}

#[tokio::test]
async fn block() -> Result<(), crate::APIError> {
    let block_number = BlockNumber::from_str(BLOCK_NUMBER).expect("could not parse {BLOCK_NUMBER} as block number");
    let block = CLIENT.block(&block_number).await?;
    assert_eq!(block_number, block.number);
    println!("Block information for {} is:\n{:#?}", block_number, block);
    Ok(())
}

#[tokio::test]
async fn block_transactions() -> Result<(), crate::APIError> {
    const BLOCK_NUMBER: &str = "0x10FB78";
    let block_number = BlockNumber::from_str(BLOCK_NUMBER).expect("could not parse {BLOCK_NUMBER} as block number");
    let transactions = CLIENT.block_transactions(&block_number).await?;
    assert_eq!(3, transactions);
    println!("Transactions for block {block_number} is {transactions}");
    Ok(())
}

#[tokio::test]
async fn call() -> Result<(), crate::APIError> {
    let address = Address::from_str("0xAEEF46DB4855E25702F8237E8f403FddcaF931C0").expect("could not parse address");
    let result = CLIENT
        .call(
            &address,
            "0x70a08231000000000000000000000000e16359506c028e51f16be38986ec5746251e9724",
            Some(Tag::Latest),
        )
        .await?;
    assert_eq!("0x00000000000000000000000000000000000000000000000000601d8888141c00", result);
    println!("Result is {result}");
    Ok(())
}

#[tokio::test]
async fn estimate_gas() -> Result<(), crate::APIError> {
    let address = Address::from_str("0xf0160428a8552ac9bb7e050d90eeade4ddd52843").expect("could not parse address");
    let gas = U64::from_str("0x5f5e0ff").unwrap().as_u64();
    let gas_price = U64::from_str("0x51da038cc").unwrap().as_u64();
    let gas = CLIENT.estimate_gas(&address, "0x4e71d92d", "0xff22", gas, gas_price).await?;
    assert_ne!(0, gas.len());
    println!("Estimated gas is {gas}");
    Ok(())
}

#[tokio::test]
async fn gas_price() -> Result<(), crate::APIError> {
    let gas_price = CLIENT.gas_price().await?;
    assert_ne!(0, gas_price);
    println!("Current gas price is {gas_price}");
    Ok(())
}

#[tokio::test]
async fn transactions() -> Result<(), crate::APIError> {
    let address = Address::from_str("0x4bd5900Cb274ef15b153066D736bf3e83A9ba44e").expect("could not parse address");
    let transactions = CLIENT.transactions(&address, Some(Tag::Latest)).await?;
    assert_eq!(112, transactions);
    println!("Transactions for {address} is {transactions}");
    Ok(())
}

#[tokio::test]
async fn uncle() -> Result<(), crate::APIError> {
    let uncle: BlockNumber = BlockNumber::from_str("0xC63276").expect("could not parse block number");
    const INDEX: u8 = 0;
    let block = CLIENT.uncle(uncle, INDEX).await?;
    println!("Uncle information for {uncle} and {INDEX} is {:?}", block);
    Ok(())
}
