mod common;

use common::get_client;

#[tokio::test]
async fn test_factory() -> Result<(), rcfe::Error> {
    get_client().await?;
    Ok(())
}
