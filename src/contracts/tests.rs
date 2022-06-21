use super::Client;
use crate::contracts::Contracts;
use crate::Address;
use once_cell::sync::Lazy;
use std::str::FromStr;

const API_KEY: &str = "";
const ADDRESS: &str = "0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413";
const BURN_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
const ERC721_ADDRESS: &str = "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D";

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new(API_KEY));

#[tokio::test]
async fn get_abi() -> Result<(), crate::APIError> {
    let address = Address::from_str(ADDRESS).expect("could not parse {ADDRESS} as address");
    let abi = CLIENT.get_abi(&address).await?;
    println!("ABI of {} is \n{:#?}", address, abi);
    Ok(())
}

#[tokio::test]
async fn get_abi_erc721() -> Result<(), crate::APIError> {
    let address = Address::from_str(ERC721_ADDRESS).expect("could not parse {ERC721_ADDRESS} as address");
    let abi = CLIENT.get_abi(&address).await?;
    println!("ABI of {} is \n{:#?}", address, abi);
    Ok(())
}

#[tokio::test]
async fn get_abi_unverified() -> Result<(), crate::APIError> {
    let address = Address::from_str(BURN_ADDRESS).expect("could not parse {BURN_ADDRESS} as address");
    if let Err(e) = CLIENT.get_abi(&address).await {
        assert!(matches!(e, crate::APIError::ContractNotVerified));
        return Ok(());
    }
    Ok(assert!(false, "expected failure"))
}

#[tokio::test]
async fn get_source_code() -> Result<(), crate::APIError> {
    let address = Address::from_str(ADDRESS).expect("could not parse {BURN_ADDRESS} as address");
    let contracts = CLIENT.get_source_code(&address).await?;
    for contract in contracts {
        assert_ne!(0, contract.source_code.len());
        assert_ne!(0, contract.contract_name.len());
        assert_ne!(0, contract.compiler_version.len());
        assert!(contract.optimization_used);
        assert!(contract.runs > 0);
        assert_ne!(0, contract.constructor_arguments.len());
        assert_ne!(0, contract.evm_version.len());
        assert_eq!(0, contract.library.len());
        assert_eq!(0, contract.license_type.len());
        assert!(!contract.proxy);
        assert_eq!(0, contract.implementation.len());
        assert_eq!(0, contract.swarm_source.len());

        println!("Contract details for {ADDRESS}:\n{:#?}", contract);
    }

    Ok(())
}

#[tokio::test]
async fn get_source_code_erc721() -> Result<(), crate::APIError> {
    let address = Address::from_str(ADDRESS).expect("could not parse {ERC721_ADDRESS} as address");
    let contracts = CLIENT.get_source_code(&address).await?;
    for contract in contracts {
        println!("Contract details for {ERC721_ADDRESS}:\n{:#?}", contract);
    }

    Ok(())
}
