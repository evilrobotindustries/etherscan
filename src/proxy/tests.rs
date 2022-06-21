use super::Client;
use crate::proxy::Proxy;
use crate::{Address, BlockNumber, Tag, TransactionHash};
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
async fn code() -> Result<(), crate::APIError> {
    let address = Address::from_str("0xf75e354c5edc8efed9b59ee9f67a80845ade7d0c").expect("could not parse as address");
    let code = CLIENT.code(&address, None).await?;
    assert_eq!(
        "0x3660008037602060003660003473273930d21e01ee25e4c219b63259d214872220a261235a5a03f21560015760206000f3",
        code
    );
    Ok(())
}

#[tokio::test]
async fn estimate_gas() -> Result<(), crate::APIError> {
    let address = Address::from_str("0xf0160428a8552ac9bb7e050d90eeade4ddd52843").expect("could not parse address");
    let value = 65314;
    let gas = 99999999;
    let gas_price = 56478107993;
    let gas = CLIENT.estimate_gas(&address, "0x4e71d92d", value, gas, gas_price).await?;
    assert_ne!(0, gas);
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
async fn storage_value() -> Result<(), crate::APIError> {
    let address = Address::from_str("0x6e03d9cce9d60f3e9f2597e13cd4c54c55330cfd").expect("could not parse as address");
    let position = 0;
    let value = CLIENT.storage_value(&address, position, None).await?;
    assert_eq!("0x0000000000000000000000000000000000000000000000000000000000000000", value);
    Ok(())
}

#[tokio::test]
async fn transaction() -> Result<(), crate::APIError> {
    let hash = TransactionHash::from_str("0xbc78ab8a9e9a0bca7d0321a27b2c03addeae08ba81ea98b03cd3dd237eabed44")
        .expect("could not parse transaction hash");
    let transaction = CLIENT.transaction(&hash).await?.unwrap();
    println!("Transaction for {hash} is {:#?}", transaction);
    Ok(())
}

#[tokio::test]
async fn pending_transaction() -> Result<(), crate::APIError> {
    let hash = TransactionHash::from_str("0x4b333e56732299bd6729a744db03ece58a3135e1bcd56e248a72da95e87972bf ")
        .expect("could not parse transaction hash");
    let transaction = CLIENT.transaction(&hash).await?;
    assert!(transaction.is_none());
    println!("Transaction for {hash} is {:#?}", transaction);
    Ok(())
}

#[tokio::test]
async fn transaction_receipt() -> Result<(), crate::APIError> {
    let hash = TransactionHash::from_str("0xadb8aec59e80db99811ac4a0235efa3e45da32928bcff557998552250fa672eb")
        .expect("could not parse transaction hash");
    let receipt = CLIENT.transaction_receipt(&hash).await?.unwrap();
    println!("Transaction receipt for {hash} is {:#?}", receipt);
    Ok(())
}

#[tokio::test]
async fn transaction_within_block() -> Result<(), crate::APIError> {
    let block_number = BlockNumber::from_str("0xC6331D").expect("could not parse transaction hash");
    const INDEX: u16 = 282;
    let transaction = CLIENT.transaction_within_block(block_number, INDEX).await?.unwrap();
    assert_eq!(
        TransactionHash::from_str("0xc7ef51f0bfe85eefbb1d4d88f5a39e82fbfc94987d8cbcb515f74d80b6e44902").unwrap(),
        transaction.hash
    );
    println!("Transaction for {block_number}[{INDEX}] is {:#?}", transaction);
    Ok(())
}

#[tokio::test]
async fn transactions() -> Result<(), crate::APIError> {
    let address = Address::from_str("0x4bd5900Cb274ef15b153066D736bf3e83A9ba44e").expect("could not parse address");
    let transactions = CLIENT.transactions(&address, Some(Tag::Latest)).await?;
    assert_eq!(113, transactions);
    println!("Transactions for {address} is {transactions}");
    Ok(())
}

#[tokio::test]
async fn uncle() -> Result<(), crate::APIError> {
    let uncle = BlockNumber::from_str("0xC63276").expect("could not parse block number");
    const INDEX: u16 = 0;
    let block = CLIENT.uncle(uncle, INDEX).await?;
    println!("Uncle information for {uncle} and {INDEX} is \n{:#?}", block);
    Ok(())
}
