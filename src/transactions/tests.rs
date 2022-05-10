use super::Client;
use crate::blocks::Closest;
use crate::{BlockNumber, TransactionHash};
use chrono::{TimeZone, Utc};
use once_cell::sync::Lazy;
use std::str::FromStr;

const API_KEY: &str = "";

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new(API_KEY));

#[tokio::test]
async fn execution_status() -> Result<(), crate::APIError> {
    let hash = TransactionHash::from_str("0x15f8e5ea1079d9a0bb04a4c58ae5fe7654b5b2b4463375ff7ffb490aa0032f3a")
        .expect("unable to parse as transaction hash");
    let status = CLIENT.execution_status(&hash).await?;
    assert!(status.is_error);
    assert_eq!("Bad jump destination", status.error_description);
    Ok(())
}

#[tokio::test]
async fn receipt_status() -> Result<(), crate::APIError> {
    let hash = TransactionHash::from_str("0x513c1ba0bebf66436b5fed86ab668452b7805593c05073eb2d51d3a52f480a76")
        .expect("unable to parse as transaction hash");
    let status = CLIENT.receipt_status(&hash).await?;
    assert!(status);
    Ok(())
}
