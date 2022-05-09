use super::Client;
use once_cell::sync::Lazy;

const API_KEY: &str = "";

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new(API_KEY));

#[tokio::test]
async fn estimate_time() -> Result<(), crate::APIError> {
    let seconds = CLIENT.estimate_time(2000000000).await?;
    assert!(seconds > 0);
    println!("Estimated time is {seconds} seconds");
    Ok(())
}

#[tokio::test]
async fn oracle() -> Result<(), crate::APIError> {
    let oracle = CLIENT.oracle().await?;
    println!("Current gas prices are {:?}", oracle);
    Ok(())
}
