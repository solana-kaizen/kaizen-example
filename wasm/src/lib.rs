use wasm_bindgen::prelude::*;
use workflow_allocator::prelude::*;
use workflow_allocator::result::Result;
use example::{client, program::ExampleHandlers, client::ExampleData, authority};
//use workflow_allocator::user::User;

#[wasm_bindgen]
pub async fn run_test(authority:Pubkey)->Result<()>{
    let tx_list = client::ExampleHandlerClient::run_test(
        &authority
    )?;
    tx_list.post().await?;
    Ok(())
}

#[wasm_bindgen]
pub async fn run_create(authority:Pubkey)->Result<Pubkey>{
    let data = ExampleData{
        msg: "hello".to_string()
    };
    let tx_list = client::ExampleHandlerClient::create(
        &authority,
        &data
    ).await?;
    let key = tx_list.target_account()?;
    tx_list.post().await?;
    Ok(key)
}

#[wasm_bindgen]
pub async fn load(pubkey:Pubkey)->Result<()>{
    if let Some(_c_ref) = load_container::<ExampleHandlers>(&pubkey).await?{
        //let data = c_ref.
        //log_trace!("c_ref: {:?}", c_ref);
    }
    Ok(())
}

#[wasm_bindgen]
pub async fn start_tests()->Result<()>{
    let authority = authority::create_custom_authority().await?;

    run_test(authority.clone()).await?;
    let key = run_create(authority.clone()).await?;
    load(key).await?;

    Ok(())
}

