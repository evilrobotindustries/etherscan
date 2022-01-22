use super::Client;
use crate::accounts::{transactions::TransactionOptions, BlockType, Page, Sort};
use once_cell::sync::Lazy;
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
    let balance = CLIENT.balance(ADDRESS, None).await?;
    assert_ne!(0f64, balance);
    println!("Balance of {} is {} ETH", ADDRESS, balance);
    Ok(())
}

#[tokio::test]
async fn balance_zero() -> Result<(), crate::APIError> {
    let balance = CLIENT.balance(UNUSED_ADDRESS, None).await?;
    assert_eq!(0f64, balance);
    Ok(())
}

#[tokio::test]
async fn balances() -> Result<(), crate::APIError> {
    let accounts = vec![ADDRESS, BURN_ADDRESS];
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
    let accounts = vec![UNUSED_ADDRESS];
    let balances = CLIENT.balances(accounts, None).await?;
    assert_eq!(1, balances.len());
    assert_eq!(UNUSED_ADDRESS, balances[0].account);
    assert_eq!(0f64, balances[0].balance);
    Ok(())
}

#[tokio::test]
async fn transactions() -> Result<(), crate::APIError> {
    let transactions = CLIENT.transactions(ADDRESS).await?;
    assert_ne!(0, transactions.len());
    println!("Address {} has {} transactions", ADDRESS, transactions.len());
    for transaction in &transactions {
        println!("{:?}", transaction);
    }
    Ok(())
}

#[tokio::test]
async fn transactions_pages() -> Result<(), crate::APIError> {
    let address = BURN_ADDRESS;
    let offset = 100;
    let transactions = CLIENT
        .transactions_with_options(address, TransactionOptions::new_page(1, offset))
        .await?;
    assert_eq!(offset as usize, transactions.len());
    let block_number = &transactions[0].block_number;

    sleep(Duration::from_secs(5)).await; // API rate limiting

    let transactions = CLIENT
        .transactions_with_options(address, TransactionOptions::new_page(2, offset))
        .await?;
    assert_eq!(offset as usize, transactions.len());
    assert_ne!(block_number, &transactions[0].block_number);

    Ok(())
}

#[tokio::test]
async fn transactions_sorts() -> Result<(), crate::APIError> {
    let address = BURN_ADDRESS;
    let offset = 1;
    let transactions = CLIENT
        .transactions_with_options(address, TransactionOptions::new_page_with_sort(1, offset, Sort::Ascending))
        .await?;
    assert_eq!(offset as usize, transactions.len());
    let time_stamp = &transactions[0].time_stamp;

    sleep(Duration::from_secs(5)).await; // API rate limiting

    let transactions = CLIENT
        .transactions_with_options(address, TransactionOptions::new_page_with_sort(1, offset, Sort::Descending))
        .await?;
    assert_eq!(offset as usize, transactions.len());
    assert!(time_stamp < &transactions[0].time_stamp);

    Ok(())
}

#[tokio::test]
async fn transactions_no_results() -> Result<(), crate::APIError> {
    let transactions = CLIENT.transactions(UNUSED_ADDRESS).await?;
    assert_eq!(0, transactions.len());
    Ok(())
}

#[tokio::test]
async fn internal_transactions() -> Result<(), crate::APIError> {
    let transactions = CLIENT.internal_transactions(ADDRESS).await?;
    assert_ne!(0, transactions.len());
    println!("Address {} has {} internal transactions", ADDRESS, transactions.len());
    for transaction in &transactions {
        println!("{:?}", transaction);
    }
    Ok(())
}

#[tokio::test]
async fn internal_transactions_for_transaction() -> Result<(), crate::APIError> {
    let transaction_hash = "0x40eb908387324f2b575b4879cd9d7188f69c8fc9d87c901b9e2daaea4b442170";
    let transactions = CLIENT.internal_transactions_for_transaction(transaction_hash).await?;
    assert_ne!(0, transactions.len());
    println!("Transaction {} has {} internal transactions", transaction_hash, transactions.len());
    for transaction in &transactions {
        println!("{:?}", transaction);
    }
    Ok(())
}

#[tokio::test]
async fn internal_transactions_no_results() -> Result<(), crate::APIError> {
    let transactions = CLIENT.internal_transactions(UNUSED_ADDRESS).await?;
    assert_eq!(0, transactions.len());
    Ok(())
}

#[tokio::test]
async fn erc20_token_transfers() -> Result<(), crate::APIError> {
    let transfers = CLIENT.erc20_token_transfers_by_address(ADDRESS).await?;
    assert_ne!(0, transfers.len());
    println!("Address {} has {} ERC20 token transfers:", ADDRESS, transfers.len(),);
    for transfer in &transfers {
        println!("{:?}", transfer);
    }
    Ok(())
}

#[tokio::test]
async fn erc20_token_transfers_no_results() -> Result<(), crate::APIError> {
    let transfers = CLIENT.erc20_token_transfers_by_address(UNUSED_ADDRESS).await?;
    assert_eq!(0, transfers.len());
    Ok(())
}

#[tokio::test]
async fn erc721_token_transfers_by_address() -> Result<(), crate::APIError> {
    let transfers = CLIENT.erc721_token_transfers_by_address(ADDRESS).await?;
    assert_ne!(0, transfers.len());
    println!("Address {} has {} ERC721 token transfers:", ADDRESS, transfers.len(),);
    for transfer in &transfers {
        println!("{:?}", transfer);
    }
    Ok(())
}

#[tokio::test]
async fn erc721_token_transfers_by_contract_address() -> Result<(), crate::APIError> {
    let transfers = CLIENT.erc721_token_transfers_by_contract_address(CONTRACT_ADDRESS).await?;
    assert_ne!(0, transfers.len());
    println!("Address {} has {} ERC721 token transfers:", CONTRACT_ADDRESS, transfers.len(),);
    for transfer in &transfers {
        println!("{:?}", transfer);
    }
    Ok(())
}

#[tokio::test]
async fn erc721_token_transfers_no_results() -> Result<(), crate::APIError> {
    let transfers = CLIENT.erc721_token_transfers_by_address(UNUSED_ADDRESS).await?;
    assert_eq!(0, transfers.len());
    Ok(())
}

#[tokio::test]
async fn blocks_mined() -> Result<(), crate::APIError> {
    let blocks = CLIENT.blocks_mined(MINER_ADDRESS, BlockType::Blocks, Page::new(1, 10)).await?;
    assert_ne!(0, blocks.len());
    println!("Address {} has mined {} blocks:", MINER_ADDRESS, blocks.len(),);
    for block in &blocks {
        println!("{:?}", block);
    }
    Ok(())
}

#[tokio::test]
async fn blocks_mined_no_results() -> Result<(), crate::APIError> {
    let blocks = CLIENT.blocks_mined(UNUSED_ADDRESS, BlockType::Uncles, Page::new(1, 10)).await?;
    assert_eq!(0, blocks.len());
    Ok(())
}
