use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {

        use std::sync::Arc;
        use kaizen::prelude::{AccountData, AccountDataReference};
        use workflow_log::*;
        use workflow_rpc::server::prelude::*;
        use kaizen::store::*;
        use kaizen::generate_random_pubkey;
        use kaizen::emulator::{EmulatorOps,Server};//server::Server;
        use thiserror::Error;
        use clap::{Parser,Subcommand};
        
        #[derive(Debug, Error)]
        enum Error {
            #[error("WebSocket error: {0}")]
            WebSocket(#[from] workflow_websocket::server::Error),
            #[error("Workflow allocator error: {0}")]
            WorkflowAllocator(String),
            #[error("I/O error: {0}")]
            IoError(#[from] std::io::Error),
        }
        
        impl From<kaizen::error::Error> for Error {
            fn from(error: kaizen::error::Error) -> Self {
                Error::WorkflowAllocator(error.to_string())
            }
        }
        
        #[derive(Debug, Parser)]
        struct Args {
            #[clap(subcommand)]
            action : Action
        }
        
        #[derive(Subcommand, Debug)]
        enum Action {
            Service { 
                #[clap(long)]
                host : Option<String>, 
                #[clap(long)]
                port : Option<u16>,
                #[clap(long)]
                purge : bool,
            },
            Test,
            Purge,
            List,
        }
        
        
        
        #[tokio::main]
        async fn main() -> Result<(),Error> {
        
            let args = Args::parse();
            match args.action {
                Action::Service { host, port, purge } => {

                    if purge {
                        println!();
                        log_info!("Purging all accounts at: {}",FileStore::default_data_folder().into_os_string().into_string().unwrap());
                        std::fs::remove_dir_all(FileStore::default_data_folder())?;    
                    }

                    println!();
                    example::init();
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~
                    kaizen::init()?;
                    kaizen::program::registry::list_entrypoints()?;
                    kaizen::container::registry::list_containers()?;
                    // ~~~~~~~~~~~~~~~~~~~~~~~~~
                    println!();
                    
                    let server = Arc::new(Server::try_new()?);
                    server.init().await?;
                    let rpc = RpcServer::new_with_encoding::<Arc<Server>,(),EmulatorOps,Id64>(Encoding::Borsh, server.clone(), server.interface().into());
        
                    let host = host.unwrap_or("127.0.0.1".to_string());
                    let port = port.unwrap_or(9393);

                    let addr = format!("{host}:{port}");
                    log_info!("Workflow emulator is listening on {}", addr);
                    rpc.listen(&addr).await?;
                },
                Action::Test => {
                    let store = FileStore::try_new()?;
                    let account_data = AccountData::new_static(generate_random_pubkey(),generate_random_pubkey());
                    let reference = Arc::new(AccountDataReference::new(account_data));
                    store.store(&reference).await?;        
                },
                Action::Purge => {
                    println!();
                    log_info!("Purging all accounts at: {}",FileStore::default_data_folder().into_os_string().into_string().unwrap());
                    std::fs::remove_dir_all(FileStore::default_data_folder())?;
                },
                Action::List => {
                    example::init();
                    kaizen::init()?;
                    let store = FileStore::try_new()?;
                    store.list().await?.to_log();
                }
            }
        
            Ok(())
        }
    } else {
        fn main() -> std::result::Result<(),String> {
            panic!("wasm32 target is not supported");
        }
    }
}
