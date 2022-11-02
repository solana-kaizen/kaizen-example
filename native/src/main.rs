
use workflow_allocator::prelude::*;
use workflow_allocator::result::Result;
// use example::{client, program::ExampleHandlers, client::ExampleData};
use example::client;
//use workflow_allocator::user::User;
// use workflow_allocator::utils;
use std::str::FromStr;
use tokio;

// async fn run_test(authority:Pubkey)->Result<()>{
//     let tx_list = client::ExampleHandlerClient::run_test(
//         &authority
//     )?;
//     tx_list.post().await?;
//     Ok(())
// }

// async fn run_create(authority:Pubkey)->Result<Pubkey>{
//     let data = ExampleData{
//         msg: "hello".to_string()
//     };
//     let tx_list = client::ExampleHandlerClient::create(
//         &authority,
//         &data
//     ).await?;
//     let key = tx_list.target_account()?;
//     tx_list.post().await?;
//     Ok(key)
// }

// async fn load(pubkey:Pubkey)->Result<()>{
//     if let Some(_c_ref) = load_container::<ExampleHandlers>(&pubkey).await?{
//         //let data = c_ref.
//         //log_trace!("c_ref: {:?}", c_ref);
//     }
//     Ok(())
// }


async fn main_impl()->Result<()>{
    let transport = Transport::try_new(
        "rpc://127.0.0.1:9393",
        TransportConfig::default()
    ).await?;
    transport.connect(true).await?;

    transport.set_custom_authority(
        Some(Pubkey::from_str("42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f")?)
    )?;

    client::run_example().await?;

    // let authority = transport.get_authority_pubkey()?;

    // transport
    //     .emulator()
    //     .expect("Missing emulator interface")
    //     .fund(
    //         &authority,
    //         &Pubkey::default(),
    //         utils::sol_to_lamports(500.0)
    //     ).await?;

    // run_test(authority.clone()).await?;
    // let key = run_create(authority.clone()).await?;
    // load(key).await?;

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



