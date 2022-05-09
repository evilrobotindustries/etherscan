use super::Client;
use once_cell::sync::Lazy;

const API_KEY: &str = "";
const BLOCK_NUMBER: &str = "0xc63251";

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new(API_KEY));

#[tokio::test]
async fn block_number() -> Result<(), crate::APIError> {
    let block_number = CLIENT.block_number().await?;
    assert_ne!(0, block_number.len());
    println!("Most recent block is {}", block_number);
    Ok(())
}

#[tokio::test]
async fn block() -> Result<(), crate::APIError> {
    let block = CLIENT.block(BLOCK_NUMBER).await?;
    //assert_ne!(0, block_number.len());
    println!("Block information for {BLOCK_NUMBER} is {:?}", block);
    Ok(())
}

#[tokio::test]
async fn block_transactions() -> Result<(), crate::APIError> {
    let transactions = CLIENT.block_transactions("0x10FB78").await?;
    assert_eq!("0x3", transactions);
    Ok(())
}

#[tokio::test]
async fn call() -> Result<(), crate::APIError> {
    let result = CLIENT
        .call(
            "0xAEEF46DB4855E25702F8237E8f403FddcaF931C0",
            "0x70a08231000000000000000000000000e16359506c028e51f16be38986ec5746251e9724",
        )
        .await?;
    assert_eq!("0x00000000000000000000000000000000000000000000000000601d8888141c00", result);
    println!("Result is {result}");
    Ok(())
}

#[tokio::test]
async fn estimate_gas() -> Result<(), crate::APIError> {
    let gas = CLIENT
        .estimate_gas(
            "0xf0160428a8552ac9bb7e050d90eeade4ddd52843",
            "0x4e71d92d",
            "0xff22",
            "0x5f5e0ff",
            "0x51da038cc",
        )
        .await?;
    assert_ne!(0, gas.len());
    println!("Estimated gas is {gas}");
    Ok(())
}

#[tokio::test]
async fn gas_price() -> Result<(), crate::APIError> {
    let gas_price = CLIENT.gas_price().await?;
    assert_ne!(0, gas_price.len());
    println!("Current gas price is {gas_price}");
    Ok(())
}

#[tokio::test]
async fn transactions() -> Result<(), crate::APIError> {
    let transactions = CLIENT.transactions("0x4bd5900Cb274ef15b153066D736bf3e83A9ba44e").await?;
    assert_eq!("0x70", transactions);
    Ok(())
}

#[tokio::test]
async fn uncle() -> Result<(), crate::APIError> {
    const UNCLE: &str = "0xC63276";
    const INDEX: &str = "0x0";

    let uncle = CLIENT.uncle(UNCLE, INDEX).await?;
    //assert_ne!(0, block_number.len());
    println!("Uncle information for {UNCLE} and {INDEX} is {:?}", uncle);
    Ok(())
}
