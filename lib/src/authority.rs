use workflow_allocator::prelude::*;
use workflow_allocator::result::Result;
use workflow_allocator::utils::sol_to_lamports;
use crate::program_id;

pub fn get_transport()->Result<Arc<Transport>>{
    Ok(Transport::global().expect("Transport is missing"))
}

pub async fn load_sol_to_authority()->Result<()>{
    let transport =  get_transport()?;
    if let Some(emulator) = transport.emulator.as_ref(){
        let balance = match transport.balance().await{
            Ok(balance)=>balance,
            Err(_)=>0
        };

        log_trace!("account balance : {}", balance);
        if balance <= sol_to_lamports(1.0) {
            let pubkey: Pubkey = transport.get_authority_pubkey_impl()?;
            emulator.fund(
                &pubkey,
                &program_id(),
                sol_to_lamports(50.0)
            ).await?;
            let balance = transport.balance().await?;
            log_trace!("account balance after funding : {}", balance);
        }
    }
    Ok(())
}

pub async fn create_custom_authority()->Result<Pubkey>{
    let authority_pubkey = generate_random_pubkey();

    get_transport()?.set_custom_authority(Some(authority_pubkey.clone()))?;
    load_sol_to_authority().await?;

    Ok(authority_pubkey)
}

pub async fn _create_identity()->Result<()>{
    /*
    log_trace!("create_identity....");
    let transport = workflow_allocator::transport::Transport::global()?;
    let authority = transport.get_authority_pubkey()?;
    let transactions = IdentityClient::create(&authority).await?;
    transactions.execute().await?;
    
    let identity = IdentityClient::load(&authority).await;

    match identity {
        Ok(Some(identity)) => {
            log_trace!("create_identity.... load_identity");
            //self.load_identity().await?;

            log_trace!("create_identity.... store");
            //self.store()?;

            Ok(identity)
        },
        Ok(None) => {
            Err(workflow_allocator::error!("Error creating identity (invalid state)").into())
        }
        Err(err) => {
            Err(workflow_allocator::error!("Error creating identity: {}",err).into())
        }
    }
    */
    Ok(())
}
