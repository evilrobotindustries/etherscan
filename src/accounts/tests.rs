use super::Client;
use crate::accounts::{transactions::TransactionOptions, BlockType, Page, Sort};
use crate::{convert, Address, BlockNumber, TransactionHash};
use once_cell::sync::Lazy;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

const API_KEY: &str = "";
const ADDRESS: &str = "0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae";
const BURN_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
const CONTRACT_ADDRESS: &str = "0x06012c8cf97bead5deae237070f9587f8e7a266d";
const MINER_ADDRESS: &str = "0x9dd134d14d1e65f84b706d6f205cd5b1cd03a46b";
const UNUSED_ADDRESS: &str = "0xCBb08a7EF0A81817dD4D018De00311B3d0cF07c6";

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new(API_KEY));

#[tokio::test]
async fn balance() -> Result<(), crate::APIError> {
    let address = Address::from_str(ADDRESS).expect("could not parse {ADDRESS} as address");
    let balance = convert::wei_to_eth(CLIENT.balance(&address, None).await?);
    assert_ne!(0f64, balance);
    println!("Balance of {} is {} ETH", address, balance);
    Ok(())
}

#[tokio::test]
async fn balance_zero() -> Result<(), crate::APIError> {
    let address = Address::from_str(UNUSED_ADDRESS).expect("could not parse {UNUSED_ADDRESS} as address");
    let balance = CLIENT.balance(&address, None).await?;
    assert_eq!(0, balance);
    Ok(())
}

#[tokio::test]
async fn balances() -> Result<(), crate::APIError> {
    let address = Address::from_str(ADDRESS).expect("could not parse {ADDRESS} as address");
    let burn_address = Address::from_str(BURN_ADDRESS).expect("could not parse {BURN_ADDRESS} as address");
    let accounts = vec![&address, &burn_address];
    let balances = CLIENT.balances(accounts, None).await?;
    assert_ne!(0, balances.len());
    println!("{} balances available", balances.len());
    for balance in &balances {
        println!("{:?}", balance);
    }
    Ok(())
}

#[tokio::test]
async fn balances_no_results() -> Result<(), crate::APIError> {
    let address = Address::from_str(UNUSED_ADDRESS).expect("could not parse {UNUSED_ADDRESS} as address");
    let balances = CLIENT.balances(vec![&address], None).await?;
    assert_eq!(1, balances.len());
    assert_eq!(address, balances[0].account);
    assert_eq!(0, balances[0].balance);
    Ok(())
}

#[tokio::test]
async fn transactions() -> Result<(), crate::APIError> {
    let address = Address::from_str(ADDRESS).expect("could not parse {ADDRESS} as address");
    let transactions = CLIENT.transactions(&address).await?;
    assert_ne!(0, transactions.len());
    println!("Address {} has {} transactions", address, transactions.len());
    assert_eq!(BlockNumber::from(54092), transactions[0].block_number);
    for transaction in &transactions {
        println!("{:?}", transaction);
    }
    Ok(())
}

#[tokio::test]
async fn transactions_pages() -> Result<(), crate::APIError> {
    let address = Address::from_str(BURN_ADDRESS).expect("could not parse {BURN_ADDRESS} as address");
    let offset = 100;
    let transactions = CLIENT
        .transactions_with_options(&address, TransactionOptions::new_page(1, offset))
        .await?;
    assert_eq!(offset as usize, transactions.len());
    let block_number = &transactions[0].block_number;

    sleep(Duration::from_secs(5)).await; // API rate limiting

    let transactions = CLIENT
        .transactions_with_options(&address, TransactionOptions::new_page(2, offset))
        .await?;
    assert_eq!(offset as usize, transactions.len());
    assert_ne!(block_number, &transactions[0].block_number);

    Ok(())
}

#[tokio::test]
async fn transactions_sorts() -> Result<(), crate::APIError> {
    let address = Address::from_str(BURN_ADDRESS).expect("could not parse {BURN_ADDRESS} as address");
    let offset = 1;
    let transactions = CLIENT
        .transactions_with_options(&address, TransactionOptions::new_page_with_sort(1, offset, Sort::Ascending))
        .await?;
    assert_eq!(offset as usize, transactions.len());
    let time_stamp = &transactions[0].time_stamp;

    sleep(Duration::from_secs(5)).await; // API rate limiting

    let transactions = CLIENT
        .transactions_with_options(&address, TransactionOptions::new_page_with_sort(1, offset, Sort::Descending))
        .await?;
    assert_eq!(offset as usize, transactions.len());
    assert!(time_stamp < &transactions[0].time_stamp);

    Ok(())
}

#[tokio::test]
async fn transactions_no_results() -> Result<(), crate::APIError> {
    let address = Address::from_str(UNUSED_ADDRESS).expect("could not parse {UNUSED_ADDRESS} as address");
    let transactions = CLIENT.transactions(&address).await?;
    assert_eq!(0, transactions.len());
    Ok(())
}

#[tokio::test]
async fn internal_transactions() -> Result<(), crate::APIError> {
    let address = Address::from_str(ADDRESS).expect("could not parse {ADDRESS} as address");
    let transactions = CLIENT.internal_transactions(&address).await?;
    assert_ne!(0, transactions.len());
    println!("Address {} has {} internal transactions", address, transactions.len());
    assert_eq!(BlockNumber::from(92038), transactions[0].block_number);
    for transaction in &transactions {
        println!("{:?}", transaction);
    }
    Ok(())
}

#[tokio::test]
async fn internal_transactions_for_transaction() -> Result<(), crate::APIError> {
    let transaction_hash = TransactionHash::from_str("0x40eb908387324f2b575b4879cd9d7188f69c8fc9d87c901b9e2daaea4b442170")
        .expect("could not parse transaction hash");
    let transactions = CLIENT.internal_transactions_for_transaction(&transaction_hash).await?;
    assert_ne!(0, transactions.len());
    println!("Transaction {} has {} internal transactions", transaction_hash, transactions.len());
    for transaction in &transactions {
        println!("{:?}", transaction);
    }
    Ok(())
}

#[tokio::test]
async fn internal_transactions_no_results() -> Result<(), crate::APIError> {
    let address = Address::from_str(UNUSED_ADDRESS).expect("could not parse {UNUSED_ADDRESS} as address");
    let transactions = CLIENT.internal_transactions(&address).await?;
    assert_eq!(0, transactions.len());
    Ok(())
}

#[tokio::test]
async fn erc20_token_balance() -> Result<(), crate::APIError> {
    let address = Address::from_str("0xe04f27eb70e025b78871a2ad7eabe85e61212761").expect("could not parse as address");
    let contract_address = Address::from_str("0x57d90b64a1a57749b0f932f1a3395792e12e7055").expect("could not parse as address");
    let balance = CLIENT.erc20_token_balance(&address, &contract_address).await?;
    assert_eq!(135499, balance);
    Ok(())
}

#[tokio::test]
async fn erc20_token_transfers() -> Result<(), crate::APIError> {
    let address = Address::from_str(ADDRESS).expect("could not parse {ADDRESS} as address");
    let transfers = CLIENT.erc20_token_transfers_by_address(&address).await?;
    assert_ne!(0, transfers.len());
    println!("Address {} has {} ERC20 token transfers", address, transfers.len(),);
    assert_eq!(BlockNumber::from(4041874), transfers[0].block_number);
    Ok(())
}

#[tokio::test]
async fn erc20_token_transfers_no_results() -> Result<(), crate::APIError> {
    let address = Address::from_str(UNUSED_ADDRESS).expect("could not parse {UNUSED_ADDRESS} as address");
    let transfers = CLIENT.erc20_token_transfers_by_address(&address).await?;
    assert_eq!(0, transfers.len());
    Ok(())
}

#[tokio::test]
async fn erc721_token_transfers_by_address() -> Result<(), crate::APIError> {
    let address = Address::from_str(ADDRESS).expect("could not parse {ADDRESS} as address");
    let transfers = CLIENT.erc721_token_transfers_by_address(&address).await?;
    assert_ne!(0, transfers.len());
    println!("Address {} has {} ERC721 token transfers", address, transfers.len(),);
    assert_eq!(BlockNumber::from(7739128), transfers[0].block_number);
    Ok(())
}

#[tokio::test]
async fn erc721_token_transfers_by_contract_address() -> Result<(), crate::APIError> {
    let address = Address::from_str(CONTRACT_ADDRESS).expect("could not parse {CONTRACT_ADDRESS} as address");
    let transfers = CLIENT.erc721_token_transfers_by_contract_address(&address).await?;
    assert_ne!(0, transfers.len());
    println!("Address {} has {} ERC721 token transfers", address, transfers.len(),);
    Ok(())
}

#[tokio::test]
async fn erc721_token_transfers_no_results() -> Result<(), crate::APIError> {
    let address = Address::from_str(UNUSED_ADDRESS).expect("could not parse {UNUSED_ADDRESS} as address");
    let transfers = CLIENT.erc721_token_transfers_by_address(&address).await?;
    assert_eq!(0, transfers.len());
    Ok(())
}

#[tokio::test]
async fn blocks_mined() -> Result<(), crate::APIError> {
    let address = Address::from_str(MINER_ADDRESS).expect("could not parse {MINER_ADDRESS} as address");
    let blocks = CLIENT.blocks_mined(&address, BlockType::Blocks, Page::new(1, 10)).await?;
    assert_ne!(0, blocks.len());
    println!("Address {} has mined {} blocks:", address, blocks.len(),);
    for block in &blocks {
        println!("{:?}", block);
    }
    Ok(())
}

#[tokio::test]
async fn blocks_mined_no_results() -> Result<(), crate::APIError> {
    let address = Address::from_str(UNUSED_ADDRESS).expect("could not parse {UNUSED_ADDRESS} as address");
    let blocks = CLIENT.blocks_mined(&address, BlockType::Uncles, Page::new(1, 10)).await?;
    assert_eq!(0, blocks.len());
    Ok(())
}
