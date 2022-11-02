
use workflow_allocator::prelude::*;
use workflow_allocator::{utils, result::Result};
use example::client;
use std::str::FromStr;
use tokio;

async fn main_impl()->Result<()>{

    let emulator_host = "rpc://127.0.0.1:9393";
    let test_validator_host = "http://127.0.0.1:8899";

    let emulator_mode = false;

    let network = if emulator_mode {
        emulator_host
    }else{
        test_validator_host
    };

    let transport = Transport::try_new(
        network,
        TransportConfig::default()
    ).await?;
    transport.connect(true).await?;

    transport.set_custom_authority(
        Some(Pubkey::from_str("42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f")?)
    )?;

    if emulator_mode{
        let authority = transport.get_authority_pubkey()?;

        transport
        .emulator()
            .expect("Missing emulator interface")
            .fund(
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



