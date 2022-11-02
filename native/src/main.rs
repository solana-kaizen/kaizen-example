
use workflow_allocator::prelude::*;
use workflow_allocator::{utils, result::Result};
use example::client;
use std::str::FromStr;
use tokio;

async fn main_impl()->Result<()>{

    let network = "rpc://127.0.0.1:9393";
    // let network = "http://127.0.0.1:8899";

    let transport = Transport::try_new(
        network,
        TransportConfig::default()
    ).await?;
    transport.connect(true).await?;

    if let Some(emulator) = transport.emulator() {
        let authority = Pubkey::from_str("42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f")?;
        transport.set_custom_authority(
            Some(authority)
        )?;
        emulator.fund(
            &authority,
            &Pubkey::default(),
            utils::sol_to_lamports(500.0)
        ).await?;
    }

    client::run_example().await?;

    Ok(())
}



#[tokio::main]
async fn main()->std::io::Result<()>{
    println!("Hello, world!");

    if let Err(err) = main_impl().await{
        println!("Error: {:?}", err);
    }

    Ok(())
}



