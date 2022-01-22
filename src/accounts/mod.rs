mod tokens;
mod transactions;

use super::Result;
use serde::de::DeserializeOwned;
use tokens::{ERC20TokenTransfer, ERC721TokenTransfer};
use transactions::{InternalTransaction, Transaction, TransactionOptions};

const ACCOUNT: &str = "account";
const ADDRESS: &str = "address";
const TAG: &str = "tag";
const PAGE: &str = "page";
const OFFSET: &str = "offset";
const SORT: &str = "sort";

pub struct Client {
    client: super::Client,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Client {
        Client {
            client: super::Client::new(api_key),
        }
    }

    /// Returns the balance of a given address.
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    /// * 'tag' - The pre-defined block parameter, which defaults to latest if not provided.
    pub async fn balance(&self, address: &str, tag: Option<Tag>) -> Result<f64> {
        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, "balance"),
            (ADDRESS, address),
            (TAG, tag.or(Some(Tag::Latest)).unwrap().to_string()),
        ];
        let b = self.client.get::<String>(parameters).await?;
        b.parse::<u128>().map(super::wei_to_eth).or(Ok(f64::NAN))
    }

    /// Returns the balances for multiple given addresses (max 20).
    ///
    /// # Arguments
    ///
    /// * 'addresses' - A list of addresses.
    /// * 'tag' - The pre-defined block parameter, which defaults to latest if not provided.
    pub async fn balances(&self, addresses: Vec<&str>, tag: Option<Tag>) -> Result<Vec<Balance>> {
        if addresses.len() > 20 {
            return Err(APIError::TooManyAddresses);
        }

        let addresses = addresses.join(",");

        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, "balancemulti"),
            (ADDRESS, addresses.as_str()),
            (TAG, tag.or(Some(Tag::Latest)).unwrap().to_string()),
        ];

        self.client.get::<Vec<Balance>>(parameters).await
    }

    /// Returns the (normal) transactions for a given address (max 10,000).
    ///
    /// # Arguments
    ///
    /// * 'address' - An address.
    pub async fn transactions(&self, address: &str) -> Result<Vec<Transaction>> {
        self.client
            .get::<Vec<Transaction>>(&[(MODULE, ACCOUNT), (ACTION, "txlist"), (ADDRESS, address)])
            .await
    }

    /// Returns the (normal) transactions for a given address (max 10,000).
    ///
    /// # Arguments
    ///
    /// * 'address' - An address.
    /// * 'options' - Additional transaction options.
    pub async fn transactions_with_options(&self, address: &str, options: TransactionOptions) -> Result<Vec<Transaction>> {
        self.get_transactions_with_options::<Transaction>("txlist", address, options).await
    }

    /// Returns the internal transactions for a given address (max 10,000).
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    pub async fn internal_transactions(&self, address: &str) -> Result<Vec<InternalTransaction>> {
        let parameters = &[(MODULE, ACCOUNT), (ACTION, "txlistinternal"), (ADDRESS, address)];
        self.client.get::<Vec<InternalTransaction>>(parameters).await
    }

    /// Returns the internal transactions performed within a transaction (max 10,000).
    ///
    /// # Arguments
    ///
    /// * 'hash' - A transaction hash.
    pub async fn internal_transactions_for_transaction(&self, hash: &str) -> Result<Vec<InternalTransaction>> {
        let parameters = &[(MODULE, ACCOUNT), (ACTION, "txlistinternal"), ("txhash", hash)];
        self.client.get::<Vec<InternalTransaction>>(parameters).await
    }

    /// Returns the internal transactions for a given address (max 10,000).
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    /// * 'options' - Additional transaction options.
    pub async fn internal_transactions_with_options(&self, address: &str, options: TransactionOptions) -> Result<Vec<InternalTransaction>> {
        self.get_transactions_with_options::<InternalTransaction>("txlistinternal", address, options)
            .await
    }

    async fn get_transactions_with_options<T: DeserializeOwned>(
        &self,
        action: &str,
        address: &str,
        options: TransactionOptions,
    ) -> Result<Vec<T>> {
        let mut parameters = vec![(MODULE, ACCOUNT), (ACTION, action), (ADDRESS, address)];

        // Add start block if provided
        let parameter;
        if let Some(start_block) = options.start_block() {
            parameter = start_block.to_string();
            parameters.push(("startblock", &parameter))
        }

        // Add end block if provided
        let parameter;
        if let Some(end_block) = options.end_block() {
            parameter = end_block.to_string();
            parameters.push(("endblock", &parameter))
        }

        // Add page if provided
        let number;
        let offset;
        if let Some(page) = options.page() {
            number = page.number.to_string();
            parameters.push((PAGE, &number));
            offset = page.offset.to_string();
            parameters.push((OFFSET, &offset));
        }

        // Add sort order if provided
        let parameter;
        if let Some(sort) = options.sort() {
            parameter = sort.to_string();
            parameters.push((SORT, &parameter))
        }

        self.client.get::<Vec<T>>(&parameters).await
    }

    /// Returns the ERC20 token transfers for a given address (max 10,000).
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    pub async fn erc20_token_transfers(&self, address: &str) -> Result<Vec<ERC20TokenTransfer>> {
        let parameters = &[(MODULE, ACCOUNT), (ACTION, "tokentx"), (ADDRESS, address)];
        self.client.get::<Vec<ERC20TokenTransfer>>(parameters).await
    }

    /// Returns the ERC721 token transfers for a given address (max 10,000).
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    pub async fn erc721_token_transfers(&self, address: &str) -> Result<Vec<ERC721TokenTransfer>> {
        let parameters = &[(MODULE, ACCOUNT), (ACTION, "tokennfttx"), (ADDRESS, address)];
        self.client.get::<Vec<ERC721TokenTransfer>>(parameters).await
    }

    /// Returns a list of blocks mined by an address.
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    pub async fn blocks_mined(&self, address: &str) -> Result<Vec<Block>> {
        let parameters = &[(MODULE, ACCOUNT), (ACTION, "getminedblocks"), (ADDRESS, address)];
        self.client.get::<Vec<Block>>(parameters).await
    }
}

use crate::{APIError, ACTION, MODULE};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Balance {
    pub account: String,
    #[serde(deserialize_with = "super::de_wei_to_eth")]
    pub balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    #[serde(alias = "blockNumber", deserialize_with = "super::de_u64_from_str")]
    pub block_number: u64,
    #[serde(alias = "timeStamp", deserialize_with = "super::de_u64_from_str")]
    pub time_stamp: u64,
    #[serde(alias = "blockReward", deserialize_with = "super::de_u128_from_str")]
    pub block_reward: u128,
}

pub struct Page {
    number: u8,
    offset: u16,
}

pub enum Sort {
    Ascending,
    Descending,
}

pub enum Tag {
    Earliest,
    Pending,
    Latest,
}

impl Sort {
    fn to_string(&self) -> &'static str {
        match self {
            Sort::Ascending => "asc",
            Sort::Descending => "desc",
        }
    }
}

impl Tag {
    fn to_string(&self) -> &'static str {
        match self {
            Tag::Latest => "latest",
            Tag::Earliest => "earliest",
            Tag::Pending => "pending",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use crate::accounts::{transactions::TransactionOptions, Sort};
    use once_cell::sync::Lazy;
    use tokio::time::{sleep, Duration};

    const API_KEY: &str = "";
    const ADDRESS: &str = "0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae";
    const BURN_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
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
        let transfers = CLIENT.erc20_token_transfers(ADDRESS).await?;
        assert_ne!(0, transfers.len());
        println!("Address {} has {} ERC20 token transfers:", ADDRESS, transfers.len(),);
        for transfer in &transfers {
            println!("{:?}", transfer);
        }
        Ok(())
    }

    #[tokio::test]
    async fn erc20_token_transfers_no_results() -> Result<(), crate::APIError> {
        let transfers = CLIENT.erc20_token_transfers(UNUSED_ADDRESS).await?;
        assert_eq!(0, transfers.len());
        Ok(())
    }

    #[tokio::test]
    async fn erc721_token_transfers() -> Result<(), crate::APIError> {
        let transfers = CLIENT.erc721_token_transfers(ADDRESS).await?;
        assert_ne!(0, transfers.len());
        println!("Address {} has {} ERC721 token transfers:", ADDRESS, transfers.len(),);
        for transfer in &transfers {
            println!("{:?}", transfer);
        }
        Ok(())
    }

    #[tokio::test]
    async fn erc721_token_transfers_no_results() -> Result<(), crate::APIError> {
        let transfers = CLIENT.erc721_token_transfers(UNUSED_ADDRESS).await?;
        assert_eq!(0, transfers.len());
        Ok(())
    }

    #[tokio::test]
    async fn blocks_mined() -> Result<(), crate::APIError> {
        let blocks = CLIENT.blocks_mined(MINER_ADDRESS).await?;
        assert_ne!(0, blocks.len());
        println!("Address {} has mined {} blocks:", MINER_ADDRESS, blocks.len(),);
        for block in &blocks {
            println!("{:?}", block);
        }
        Ok(())
    }

    #[tokio::test]
    async fn blocks_mined_no_results() -> Result<(), crate::APIError> {
        let blocks = CLIENT.blocks_mined(UNUSED_ADDRESS).await?;
        assert_eq!(0, blocks.len());
        Ok(())
    }
}
