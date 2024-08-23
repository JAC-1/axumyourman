#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:3000")?;
    hc.do_get("/").await?.print().await?;

    Ok(())
}
