use kaizen::prelude::*;

pub mod program {
    use kaizen::container::Utf8String;

    use super::*;

    // simple program handler with test function
    pub struct ExampleHandler;

    impl ExampleHandler {
        pub fn test(_ctx: &ContextReference) -> ProgramResult {
            log_trace!("hello handler test");
            Ok(())
        }
    }

    // declare this struct as a handler
    // ...declare test function
    // this macro builds a small function table
    // that is accessible program and client-side
    declare_handlers!(ExampleHandler, [ExampleHandler::test,]);

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // simple container example
    // this example uses a container, but also
    // demonstrates how static methods on any
    // struct can be used as handler endpoints

    pub enum ContainerTypes {
        ExampleContainer = 1,
    }

    // data passed to the create() function
    #[derive(Clone, BorshSerialize, BorshDeserialize)]
    pub struct CreationData {
        pub msg: String,
        pub data: RecordArgs,
    }

    #[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
    pub struct RecordArgs {
        pub int64: u64,
        pub int32: u32,
        pub pubkey: Pubkey,
        pub int8: u8,
    }

    // please note that this data structure is packed! (repr(packed))
    // as such, you can not derive Debug trait
    // however, Meta trait provided by workflow-allocator
    // creates unaligned property access functions
    // such as x.get_byte() and x.set_int64()
    // this is needed only if the structure is unaligned
    #[derive(Meta, Clone, Copy)]
    #[repr(packed)]
    pub struct RecordData {
        pub int8: u8,
        pub int32: u32,
        pub int64: u64,
        pub pubkey: Pubkey,
    }

    impl From<RecordArgs> for RecordData {
        fn from(args: RecordArgs) -> Self {
            log_trace!("{:?}", args);
            RecordData {
                int8: args.int8,
                int32: args.int32,
                int64: args.int64,
                pubkey: args.pubkey,
            }
        }
    }

    #[container(ContainerTypes::ExampleContainer)]
    pub struct ExampleContainer<'info, 'refs> {
        pub message: Utf8String<'info, 'refs>,
        pub records: Array<RecordData, 'info, 'refs>,
    }

    impl<'info, 'refs> ExampleContainer<'info, 'refs> {
        pub fn test(_ctx: &ContextReference) -> ProgramResult {
            log_trace!("hello container test!");
            Ok(())
        }

        pub fn create(ctx: &ContextReference) -> ProgramResult {
            let args = CreationData::try_from_slice(ctx.instruction_data)?;
            let allocation_args = AccountAllocationArgs::default();

            // pre-calculate additional data needed for the account to avoid realloc()
            // of the account during the record insert operation
            let extra_data = std::mem::size_of::<RecordData>() + args.msg.as_bytes().len();
            let container = ExampleContainer::try_allocate(ctx, &allocation_args, extra_data)?;

            // following operations are unsafe as they may result in segment resizing
            // since various APIs offer direct slice access to segment data, resizing
            // may result in shifts of underlying data.  Avoid retaining slices for
            // extended period of times.  On their own, these functions are perfectly safe!
            //
            // Example of problematic code:
            //      ... assume two arrays sets records_a (first) and records_b (second)
            //      let slice_b = container.records_b.as_slice(); <-- get slice from records_b
            //      container.records_a.try_insert(&args.data)?;  <-- insert into records_a
            //      let value_from_b = slice_b[0]; <-- may point to invalid data
            //      ... this can also be avoided by resizing records_a before taking it's slice
            //
            unsafe {
                // since we pre-calculated record allocation at container creation phase
                // we can call try_allocate() that will skip realloc and return mut reference
                // to what would be a newly allocated element
                let record_data_dst = container.records.try_allocate(false)?;
                *record_data_dst = args.data.into();

                // alternatively, you can just insert
                // let record_data_src: RecordData = args.data.into();
                // container.records.try_insert(&record_data_src)?;
                container.message.store(&args.msg)?;

                ctx.sync_rent(container.account(), &RentCollector::default())?;
            }

            Ok(())
        }
    }

    declare_handlers!(
        ExampleContainer::<'info, 'refs>,
        [ExampleContainer::test, ExampleContainer::create,]
    );
}

#[cfg(not(target_os = "solana"))]
pub mod client {
    use super::*;
    use kaizen::{result::Result, utils};
    use std::str::FromStr;
    use wasm_bindgen::prelude::*;
    pub struct ExampleHandlerClient;
    use rand;
    declare_client!(program::ExampleHandler, ExampleHandlerClient);

    impl ExampleHandlerClient {
        pub async fn run_test(authority: &Pubkey) -> Result<TransactionList> {
            let builder =
                ExampleHandlerClient::execution_context_for(program::ExampleHandler::test)
                    .with_authority(authority)
                    .seal()?;

            let transaction = Transaction::new_without_accounts(
                "Container test", //.to_string(),
                builder.try_into()?,
            );

            Ok(TransactionList::new(vec![transaction]))
        }
    }

    pub struct ExampleContainerClient;
    declare_client!(program::ExampleContainer, ExampleContainerClient);

    impl ExampleContainerClient {
        pub async fn create(
            authority: &Pubkey,
            data: &program::CreationData,
        ) -> Result<TransactionList> {
            let random_seed = rand::random::<[u8; 8]>();
            let builder = Self::execution_context_for(program::ExampleContainer::create)
                .with_authority(authority)
                // templates can be constructed with different sources
                // authority or authority+suffixes
                // identity, which retains count of accounts, allowing to create a linear chain
                // collections - in-container key collections
                .with_generic_account_templates_with_custom_suffixes(&[&random_seed])
                .with_instruction_data(&data.try_to_vec()?)
                .seal()?;

            // there are unspoken rules about create functions always supplying the account
            // they are creating as a first account in the account list (during builder phase)
            let accounts = builder.gather_accounts(Some(Gather::Authority), None)?;

            // transactions by default are lists of transactions containing affecting keys
            // this allows to detect an intersection of the affected accounts and queue
            // transactions that are relevant to one another; the list generated by
            // gather_accounts() does not include authority or identity keys as every
            // transaction contains them, so manual logic may still need to apply
            let transaction = Transaction::new_with_accounts(
                format!("Creating example account {}", accounts[0]).as_str(),
                accounts,
                builder.try_into()?,
            );

            Ok(TransactionList::new(vec![transaction]))
        }
    }

    #[wasm_bindgen]
    pub async fn run_example() -> Result<()> {
        let transport = Transport::global()?;
        if let Some(emulator) = transport.emulator() {
            let authority = Pubkey::from_str("42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f")?;
            transport.set_custom_authority(Some(authority))?;
            emulator
                .fund(
                    &authority,
                    &Pubkey::default(),
                    utils::sol_to_lamports(500.0),
                )
                .await?;
        }

        let authority = transport.get_authority_pubkey()?;

        let tx = client::ExampleHandlerClient::run_test(&authority).await?;
        tx.execute().await?;

        let pubkey = Pubkey::from_str("9ZNTfG4NyQgxy2SWjSiQoUyBPEvXT2xo7fKc5hPYYJ7b")?;
        let data = program::CreationData {
            msg: "hello container".to_string(),
            data: program::RecordArgs {
                int8: 1,
                int32: 2,
                int64: 3,
                pubkey,
            },
        };
        let tx = client::ExampleContainerClient::create(&authority, &data).await?;
        let target_account_pubkey = tx.target_account()?;
        tx.execute().await?;

        // load created container
        // there are different "loader" functions available for loading
        // AccountInfos (available via AccountDataReference) and
        // containers (available via ContainerReferences::<C>)
        // the framework maintains in-application cache, so once loaded, accounts are
        // available for the duration of runtime and need to be "reloaded" if one wants
        // to refresh their contents.
        // When fetching accounts, multiple lookups for the same account are fine as the future
        // will stall and wait if it detects an already ongoing lookup for the same pubkey.
        let container = load_container::<program::ExampleContainer>(&target_account_pubkey)
            .await?
            .expect("¯\\_(ツ)_/¯");

        let message = container.message.to_string();
        let record = container.records.try_get_at(0)?;
        let int8 = record.get_int8();
        let int32 = record.get_int32();
        let int64 = record.get_int64();
        let incoming_pubkey = record.get_pubkey();

        log_trace!("container data - message: {message} int8: {int8} int32: {int32} int64: {int64} pubkey: {incoming_pubkey}");

        assert_eq!(int8, 1);
        assert_eq!(int32, 2);
        assert_eq!(int64, 3);
        assert_eq!(pubkey, incoming_pubkey);

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use kaizen::result::Result;
    use std::str::FromStr;

    #[async_std::test]
    async fn example_test() -> Result<()> {
        // Workflow Allocator framework needs to be initialized client-side
        // initialization builds a local program registry and container registry
        // program registry allows the transport to map different entry points
        // (you can have multiple entry points, just make sure to expose only
        // one to the program) and container registry maps container lookup tables
        // these are done differently in native and wasm.
        kaizen::init()?;

        //
        // each client (native, wasm) creates its own variant of transport
        // transport provides a common interface to execute and lookup accounts via different
        // implementations on each platform
        // each platform can run in 3 modes:
        //    - InProc - emulator() in the current process
        //      useful for prototyping and various types of unit tests
        //    - Emulator - emulator() in a remote server instance
        //      useful for testing multi-user interaction
        //    - Validator - running against the real Solana validator
        //
        Transport::try_new_for_unit_tests(
            crate::program_id(),
            Some(Pubkey::from_str(
                "42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f",
            )?),
            TransportConfig::default(),
        )
        .await?;

        client::run_example().await?;

        Ok(())
    }
}

declare_program!(
    "example",
    "5UAQGzYRWKEgdbpZCqoUjKDKiWpNbHeataWknRpvswEH",
    [program::ExampleHandler, program::ExampleContainer,]
);
