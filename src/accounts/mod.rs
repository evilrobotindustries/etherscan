use super::Result;
use crate::accounts::tokens::TokenOptions;
use crate::{APIError, Address, BlockNumber, Client, Tag, TransactionHash, TypeExtensions, ACTION, ADDRESS, MODULE, TAG};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr, TimestampSecondsWithFrac};
use tokens::{ERC20TokenTransfer, ERC721TokenTransfer};
use transactions::{InternalTransaction, Transaction, TransactionOptions};

#[cfg(test)]
mod tests;
mod tokens;
mod transactions;

const ACCOUNT: &str = "account";
const CONTRACT_ADDRESS: &str = "contractaddress";
const END_BLOCK: &str = "endblock";
const ERC20_TOKEN_TRANSFERS: &str = "tokentx";
const ERC721_TOKEN_TRANSFERS: &str = "tokennfttx";
const INTERNAL_TRANSACTIONS: &str = "txlistinternal";
const OFFSET: &str = "offset";
const PAGE: &str = "page";
const SORT: &str = "sort";
const START_BLOCK: &str = "startblock";
const TRANSACTIONS: &str = "txlist";

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Accounts {
    /// Returns the balance of a given address in wei.
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    /// * 'tag' - The pre-defined block parameter, which defaults to latest if not provided.
    async fn balance(&self, address: &Address, tag: Option<Tag>) -> Result<u128>;

    /// Returns the balances for multiple given addresses (max 20).
    ///
    /// # Arguments
    ///
    /// * 'addresses' - A list of addresses.
    /// * 'tag' - The pre-defined block parameter, which defaults to latest if not provided.
    async fn balances(&self, addresses: Vec<&Address>, tag: Option<Tag>) -> Result<Vec<Balance>>;

    /// Returns the (normal) transactions for a given address (max 10,000).
    ///
    /// # Arguments
    ///
    /// * 'address' - An address.
    ///
    /// **Note:** This API endpoint returns a maximum of 10,000 records only.
    async fn transactions(&self, address: &Address) -> Result<Vec<Transaction>>;

    /// Returns the (normal) transactions for a given address (max 10,000).
    ///
    /// # Arguments
    ///
    /// * 'address' - An address.
    /// * 'options' - Additional options.
    async fn transactions_with_options(&self, address: &Address, options: TransactionOptions) -> Result<Vec<Transaction>>;

    /// Returns the internal transactions for a given address (max 10,000).
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    ///
    /// **Note:** This API endpoint returns a maximum of 10,000 records only.
    async fn internal_transactions(&self, address: &Address) -> Result<Vec<InternalTransaction>>;

    /// Returns the internal transactions performed within a transaction (max 10,000).
    ///
    /// # Arguments
    ///
    /// * 'hash' - A transaction hash.
    ///
    /// **Note:** This API endpoint returns a maximum of 10,000 records only.
    async fn internal_transactions_for_transaction(&self, hash: &TransactionHash) -> Result<Vec<InternalTransaction>>;

    /// Returns the internal transactions for a given address (max 10,000).
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    /// * 'options' - Additional options.
    ///
    /// **Note:** This API endpoint returns a maximum of 10,000 records only.
    async fn internal_transactions_with_options(&self, address: &Address, options: TransactionOptions) -> Result<Vec<InternalTransaction>>;

    /// Returns the current balance of an ERC-20 token of an address.
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    /// * 'contract_address' - A contract address
    async fn erc20_token_balance(&self, address: &Address, contract_address: &Address) -> Result<u128>;

    /// Returns the ERC20 token transfers for a given address and contract address.
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    /// * 'contract_address' - A contract address
    async fn erc20_token_transfers(&self, address: &Address, contract_address: &Address) -> Result<Vec<ERC20TokenTransfer>>;

    /// Returns the ERC20 token transfers for a given address.
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    async fn erc20_token_transfers_by_address(&self, address: &Address) -> Result<Vec<ERC20TokenTransfer>>;

    /// Returns the ERC20 token transfers for a given contract address.
    ///
    /// # Arguments
    ///
    /// * 'contract_address' - A contract address
    async fn erc20_token_transfers_by_contract_address(&self, contract_address: &Address) -> Result<Vec<ERC20TokenTransfer>>;

    /// Returns the ERC20 token transfers based on the supplied options.
    ///
    /// # Arguments
    ///
    /// * 'options' - The token request options.
    async fn erc20_token_transfers_with_options<'a>(&self, options: TokenOptions<'a>) -> Result<Vec<ERC20TokenTransfer>>;

    /// Returns the ERC721 token transfers for a given address and contract address.
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    /// * 'contract_address' - A contract address
    async fn erc721_token_transfers(&self, address: &Address, contract_address: &Address) -> Result<Vec<ERC721TokenTransfer>>;

    /// Returns the ERC721 token transfers for a given address.
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    async fn erc721_token_transfers_by_address(&self, address: &Address) -> Result<Vec<ERC721TokenTransfer>>;

    /// Returns the ERC721 token transfers for a given contract address.
    ///
    /// # Arguments
    ///
    /// * 'contract_address' - A contract address
    async fn erc721_token_transfers_by_contract_address(&self, contract_address: &Address) -> Result<Vec<ERC721TokenTransfer>>;

    /// Returns the ERC721 token transfers based on the supplied options.
    ///
    /// # Arguments
    ///
    /// * 'options' - The token request options.
    async fn erc721_token_transfers_with_options<'a>(&self, options: TokenOptions<'a>) -> Result<Vec<ERC721TokenTransfer>>;

    /// Returns a list of blocks mined by an address.
    ///
    /// # Arguments
    ///
    /// * 'address' - An address
    async fn blocks_mined(&self, address: &Address, block_type: BlockType, page: Page) -> Result<Vec<Block>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Accounts for Client {
    async fn balance(&self, address: &Address, tag: Option<Tag>) -> Result<u128> {
        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, "balance"),
            (ADDRESS, &TypeExtensions::format(address)),
            (TAG, tag.or(Some(Tag::Latest)).unwrap().to_string()),
        ];
        self.get::<String>(parameters).await.map(|v| v.parse::<u128>().unwrap_or(0))
    }

    async fn balances(&self, addresses: Vec<&Address>, tag: Option<Tag>) -> Result<Vec<Balance>> {
        if addresses.len() > 20 {
            return Err(APIError::TooManyAddresses);
        }

        let addresses = addresses
            .iter()
            .map(|a| TypeExtensions::format(*a))
            .collect::<Vec<String>>()
            .join(",");

        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, "balancemulti"),
            (ADDRESS, addresses.as_str()),
            (TAG, tag.or(Some(Tag::Latest)).unwrap().to_string()),
        ];

        self.get::<Vec<Balance>>(parameters).await
    }

    async fn transactions(&self, address: &Address) -> Result<Vec<Transaction>> {
        self.get::<Vec<Transaction>>(&[
            (MODULE, ACCOUNT),
            (ACTION, TRANSACTIONS),
            (ADDRESS, &TypeExtensions::format(address)),
        ])
        .await
    }

    async fn transactions_with_options(&self, address: &Address, options: TransactionOptions) -> Result<Vec<Transaction>> {
        self.get_transactions_with_options::<Transaction>(TRANSACTIONS, address, options)
            .await
    }

    async fn internal_transactions(&self, address: &Address) -> Result<Vec<InternalTransaction>> {
        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, INTERNAL_TRANSACTIONS),
            (ADDRESS, &TypeExtensions::format(address)),
        ];
        self.get::<Vec<InternalTransaction>>(parameters).await
    }

    async fn internal_transactions_for_transaction(&self, hash: &TransactionHash) -> Result<Vec<InternalTransaction>> {
        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, INTERNAL_TRANSACTIONS),
            ("txhash", &TypeExtensions::format(hash)),
        ];
        self.get::<Vec<InternalTransaction>>(parameters).await
    }

    async fn internal_transactions_with_options(&self, address: &Address, options: TransactionOptions) -> Result<Vec<InternalTransaction>> {
        self.get_transactions_with_options::<InternalTransaction>(INTERNAL_TRANSACTIONS, address, options)
            .await
    }

    async fn erc20_token_balance(&self, address: &Address, contract_address: &Address) -> Result<u128> {
        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, "tokenbalance"),
            (ADDRESS, &TypeExtensions::format(address)),
            (CONTRACT_ADDRESS, &TypeExtensions::format(contract_address)),
        ];
        self.get::<String>(parameters).await.map(|v| v.parse::<u128>().unwrap_or(0))
    }

    async fn erc20_token_transfers(&self, address: &Address, contract_address: &Address) -> Result<Vec<ERC20TokenTransfer>> {
        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, ERC20_TOKEN_TRANSFERS),
            (ADDRESS, &TypeExtensions::format(address)),
            (CONTRACT_ADDRESS, &TypeExtensions::format(contract_address)),
        ];
        self.get::<Vec<ERC20TokenTransfer>>(parameters).await
    }

    async fn erc20_token_transfers_by_address(&self, address: &Address) -> Result<Vec<ERC20TokenTransfer>> {
        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, ERC20_TOKEN_TRANSFERS),
            (ADDRESS, &TypeExtensions::format(address)),
        ];
        self.get::<Vec<ERC20TokenTransfer>>(parameters).await
    }

    async fn erc20_token_transfers_by_contract_address(&self, contract_address: &Address) -> Result<Vec<ERC20TokenTransfer>> {
        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, ERC20_TOKEN_TRANSFERS),
            (CONTRACT_ADDRESS, &TypeExtensions::format(contract_address)),
        ];
        self.get::<Vec<ERC20TokenTransfer>>(parameters).await
    }

    async fn erc20_token_transfers_with_options<'a>(&self, options: TokenOptions<'a>) -> Result<Vec<ERC20TokenTransfer>> {
        self.get_tokens_with_options::<ERC20TokenTransfer>(ERC20_TOKEN_TRANSFERS, options)
            .await
    }

    async fn erc721_token_transfers(&self, address: &Address, contract_address: &Address) -> Result<Vec<ERC721TokenTransfer>> {
        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, ERC721_TOKEN_TRANSFERS),
            (ADDRESS, &TypeExtensions::format(address)),
            (CONTRACT_ADDRESS, &TypeExtensions::format(contract_address)),
        ];
        self.get::<Vec<ERC721TokenTransfer>>(parameters).await
    }

    async fn erc721_token_transfers_by_address(&self, address: &Address) -> Result<Vec<ERC721TokenTransfer>> {
        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, ERC721_TOKEN_TRANSFERS),
            (ADDRESS, &TypeExtensions::format(address)),
        ];
        self.get::<Vec<ERC721TokenTransfer>>(parameters).await
    }

    async fn erc721_token_transfers_by_contract_address(&self, contract_address: &Address) -> Result<Vec<ERC721TokenTransfer>> {
        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, ERC721_TOKEN_TRANSFERS),
            (CONTRACT_ADDRESS, &TypeExtensions::format(contract_address)),
        ];
        self.get::<Vec<ERC721TokenTransfer>>(parameters).await
    }

    async fn erc721_token_transfers_with_options<'a>(&self, options: TokenOptions<'a>) -> Result<Vec<ERC721TokenTransfer>> {
        self.get_tokens_with_options::<ERC721TokenTransfer>(ERC721_TOKEN_TRANSFERS, options)
            .await
    }

    async fn blocks_mined(&self, address: &Address, block_type: BlockType, page: Page) -> Result<Vec<Block>> {
        let block_type = block_type.to_string();
        let page = page.to_string();
        let parameters = &[
            (MODULE, ACCOUNT),
            (ACTION, "getminedblocks"),
            (ADDRESS, &TypeExtensions::format(address)),
            ("blocktype", &block_type),
            (PAGE, &page.0),
            (OFFSET, &page.1),
        ];
        self.get::<Vec<Block>>(parameters).await
    }
}

impl Client {
    async fn get_transactions_with_options<T: DeserializeOwned>(
        &self,
        action: &str,
        address: &Address,
        options: TransactionOptions,
    ) -> Result<Vec<T>> {
        let address = &TypeExtensions::format(address);
        let mut parameters = vec![(MODULE, ACCOUNT), (ACTION, action), (ADDRESS, address)];

        // Add start block if provided
        let parameter;
        if let Some(start_block) = options.start_block() {
            parameter = start_block.to_string();
            parameters.push((START_BLOCK, &parameter))
        }

        // Add end block if provided
        let parameter;
        if let Some(end_block) = options.end_block() {
            parameter = end_block.to_string();
            parameters.push((END_BLOCK, &parameter))
        }

        // Add page if provided
        let parameter;
        if let Some(page) = options.page() {
            parameter = page.to_string();
            parameters.push((PAGE, &parameter.0));
            parameters.push((OFFSET, &parameter.1));
        }

        // Add sort order if provided
        let parameter;
        if let Some(sort) = options.sort() {
            parameter = sort.to_string();
            parameters.push((SORT, &parameter))
        }

        self.get::<Vec<T>>(&parameters).await
    }

    async fn get_tokens_with_options<'a, T: DeserializeOwned>(&self, action: &str, options: TokenOptions<'a>) -> Result<Vec<T>> {
        let mut parameters = vec![(MODULE, ACCOUNT), (ACTION, action)];

        // Add address if provided
        if let Some(address) = options.address() {
            parameters.push((ADDRESS, address))
        }

        // Add contract address if provided
        if let Some(contract_address) = options.contract_address() {
            parameters.push((ADDRESS, contract_address))
        }

        // Add page if provided
        let parameter;
        if let Some(page) = options.page() {
            parameter = page.to_string();
            parameters.push((PAGE, &parameter.0));
            parameters.push((OFFSET, &parameter.1));
        }

        // Add start block if provided
        let parameter;
        if let Some(start_block) = options.start_block() {
            parameter = start_block.to_string();
            parameters.push((START_BLOCK, &parameter))
        }

        // Add end block if provided
        let parameter;
        if let Some(end_block) = options.end_block() {
            parameter = end_block.to_string();
            parameters.push((END_BLOCK, &parameter))
        }

        // Add sort order if provided
        let parameter;
        if let Some(sort) = options.sort() {
            parameter = sort.to_string();
            parameters.push((SORT, &parameter))
        }

        self.get::<Vec<T>>(&parameters).await
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Balance {
    pub account: Address,
    #[serde_as(as = "DisplayFromStr")]
    pub balance: u128,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(deserialize_with = "crate::de_string_to_block_number")]
    pub block_number: BlockNumber,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    pub time_stamp: DateTime<Utc>,
    #[serde_as(as = "DisplayFromStr")]
    pub block_reward: u128,
}

pub enum BlockType {
    /// Canonical blocks
    Blocks,
    // Uncle blocks
    Uncles,
}

impl BlockType {
    fn to_string(&self) -> &'static str {
        match self {
            BlockType::Blocks => "blocks",
            BlockType::Uncles => "uncles",
        }
    }
}

pub struct Page {
    number: u8,
    offset: u16,
}

pub enum Sort {
    Ascending,
    Descending,
}

impl Page {
    #![allow(dead_code)]
    fn new(number: u8, offset: u16) -> Page {
        Page { number, offset }
    }

    fn to_string(&self) -> (String, String) {
        (self.number.to_string(), self.offset.to_string())
    }
}

impl Sort {
    fn to_string(&self) -> &'static str {
        match self {
            Sort::Ascending => "asc",
            Sort::Descending => "desc",
        }
    }
}
