use example::client;
use kaizen::prelude::*;
use kaizen::result::Result;

async fn main_impl() -> Result<()> {
    let network = "rpc://127.0.0.1:9393";
    // let network = "http://127.0.0.1:8899";

    let transport = Transport::try_new(network, TransportConfig::default()).await?;
    transport.connect(true).await?;

    client::run_example().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    if let Err(err) = main_impl().await {
        println!("Error: {err:?}");
    }

    Ok(())
}
