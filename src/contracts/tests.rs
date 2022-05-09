use super::Client;
use crate::contracts::Descriptor;
use once_cell::sync::Lazy;

const API_KEY: &str = "";
const ADDRESS: &str = "0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413";
const BURN_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
const ERC721_ADDRESS: &str = "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D";

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new(API_KEY));

#[tokio::test]
async fn get_abi() -> Result<(), crate::APIError> {
    let abi = CLIENT.get_abi(ADDRESS).await?;
    assert!(abi.len() > 0);
    println!("ABI of {} is \n{:#?}", ADDRESS, abi);
    Ok(())
}

#[tokio::test]
async fn get_abi_erc721() -> Result<(), crate::APIError> {
    let abi = CLIENT.get_abi(ERC721_ADDRESS).await?;
    assert!(abi.len() > 0);
    println!("ABI of {} is \n{:#?}", ERC721_ADDRESS, abi);
    Ok(())
}

#[tokio::test]
async fn get_abi_unverified() -> Result<(), crate::APIError> {
    if let Err(e) = CLIENT.get_abi(BURN_ADDRESS).await {
        assert!(matches!(e, crate::APIError::ContractNotVerified));
        return Ok(());
    }
    Ok(assert!(false, "expected failure"))
}

#[tokio::test]
async fn get_abi_invalid_address() -> Result<(), crate::APIError> {
    if let Err(e) = CLIENT.get_abi("0x1").await {
        assert!(matches!(e, crate::APIError::InvalidAddress));
        return Ok(());
    }
    Ok(assert!(false, "expected failure"))
}

#[tokio::test]
async fn get_source_code() -> Result<(), crate::APIError> {
    let contracts = CLIENT.get_source_code(ADDRESS).await?;
    for contract in contracts {
        println!("Contract details for {ADDRESS}:");

        assert_ne!(0, contract.source_code.len());
        //println!("Source code of {} is {}", ADDRESS, contract.source_code);

        assert_ne!(0, contract.abi.len());
        println!("ABI:                      {}", contract.abi);

        assert_ne!(0, contract.contract_name.len());
        println!("Contract name:            {}", contract.contract_name);

        assert_ne!(0, contract.compiler_version.len());
        println!("Compiler version:         {}", contract.compiler_version);

        assert!(contract.optimization_used);
        assert!(contract.runs > 0);
        println!(
            "Optimisation Enabled:     {} with {} runs",
            contract.optimization_used, contract.runs
        );

        assert_ne!(0, contract.constructor_arguments.len());
        println!("Constructor arguments:    {}", contract.constructor_arguments);

        assert_ne!(0, contract.evm_version.len());
        println!("EVM version:              {}", contract.evm_version);

        assert_eq!(0, contract.library.len());
        println!("Library:                  {}", contract.library);

        assert_eq!(0, contract.license_type.len());
        println!("License type:             {}", contract.license_type);

        assert!(!contract.proxy);
        println!("Proxy:                    {}", contract.proxy);

        assert_eq!(0, contract.implementation.len());
        println!("Implementation:           {}", contract.implementation);

        assert_eq!(0, contract.swarm_source.len());
        println!("Swarm source:             {}", contract.swarm_source);
    }

    Ok(())
}

#[tokio::test]
async fn get_source_code_erc721() -> Result<(), crate::APIError> {
    let contracts = CLIENT.get_source_code(ERC721_ADDRESS).await?;
    for contract in contracts {
        println!("Contract details for {ERC721_ADDRESS}:\n{:#?}", contract);
    }

    Ok(())
}
