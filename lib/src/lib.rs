use workflow_allocator::prelude::*;
#[allow(unused_imports)]
use workflow_allocator::result::Result;

#[cfg(not(target_arch = "bpf"))]
pub mod authority;

pub mod program {
    use super::*;

    pub enum ContainerTypes{
        ExampleHandlers = 1,
        ExampleContainer
    }

    #[container(ContainerTypes::ExampleHandlers)]
    pub struct ExampleHandlers<'info,'refs>{

    }
    
    impl<'info,'refs> ExampleHandlers<'info,'refs>{

        pub fn test(_ctx: &ContextReference) -> ProgramResult {
            log_trace!("Handlers: hello");
            Ok(())
        }
        pub fn create(_ctx: &ContextReference) -> ProgramResult {
            log_trace!("Handlers: hello create ");
            Ok(())
        }
    }

    declare_handlers! (ExampleHandlers::<'info,'refs>, [
        ExampleHandlers::test,
        ExampleHandlers::create,
    ]);
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    #[container(ContainerTypes::ExampleContainer)]
    pub struct ExampleContainer<'info,'refs> {

    }

    impl<'info,'refs> ExampleContainer<'info,'refs> {

        pub fn test(_ctx: &ContextReference) -> ProgramResult {
            log_trace!("hello");
            Ok(())
        }

        pub fn create(_ctx: &ContextReference) -> ProgramResult {
            log_trace!("hello create");
            Ok(())
        }
    }


    declare_handlers! (ExampleContainer::<'info,'refs>, [
        ExampleContainer::test,
    ]);
    
    
}

#[cfg(not(target_arch = "bpf"))]
pub mod client {
    use super::*;
    use borsh::*;

    pub struct ExampleHandlerClient;
    declare_client!(super::program::ExampleHandlers, ExampleHandlerClient);

    #[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
    pub struct ExampleData {
        pub msg: String
    }

    impl ExampleHandlerClient {

        pub fn run_test(
            authority:&Pubkey,
            //identity:&Pubkey
        ) -> Result<TransactionList> {
            let builder = ExampleHandlerClient::execution_context_for(
                program::ExampleHandlers::test
            )
                .with_authority(authority)
                //.with_identity(identity)
                .seal()?;

            let transaction = Transaction::new_without_accounts(
                format!("Container test").as_str(),
                builder.try_into()?
            );

            Ok(TransactionList::new(vec![transaction]))
        }

        pub async fn create(
            //user : &User,
            authority:&Pubkey,
            data : &ExampleData
        ) -> Result<TransactionList> {
    
            let builder = Self::execution_context_for(program::ExampleHandlers::create)
                //.with_user(user.into())
                .with_authority(authority)
                //.with_identity_collections(&[(true, program::ContainerTypes::ExampleHandlers as u32)]).await?
                .with_account_templates(1)
                .with_instruction_data(&data.try_to_vec()?)
                .seal()?;
    
            let accounts = builder.gather_accounts(Some(Gather::Authority),None)?;
    
            let transaction = Transaction::new_with_accounts(
                format!("Creating example account {}", accounts[0]).as_str(),
                accounts,
                builder.try_into()?
            );
    
            Ok(TransactionList::new(vec![transaction]))
        }
    

    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[async_std::test]
    async fn example_test() -> Result<()> {
        workflow_allocator::init()?;

        Ok(())
    }

}


declare_program!("example", "F9SsGPgxpBdTyiZA41X1HYLR5QtcXnNBvhoE374DWhjg",[
    program::ExampleHandlers
]); 

