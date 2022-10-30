use workflow_allocator::prelude::*;
use workflow_allocator::result::Result;

pub mod program {
    use super::*;

    pub struct ExampleHandlers { }
    
    impl ExampleHandlers {

        pub fn test(ctx: &ContextReference) -> ProgramResult {
            log_trace!("hello");
            Ok(())
        }
    }

    declare_handlers! (ExampleHandlers, [
        ExampleHandlers::test,
    ]);
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    #[container(123)]
    pub struct ExampleContainer<'info,'refs> {

    }

    impl<'info,'refs> ExampleContainer<'info,'refs> {

        pub fn test(ctx: &ContextReference) -> ProgramResult {
            log_trace!("hello");
            Ok(())
        }
    }


    declare_handlers! (ExampleContainer::<'info,'refs>, [
        ExampleContainer::test,
    ]);
    
    
}

pub mod client {
    use super::*;

    pub struct ExampleHandlerClient;
    declare_client!(super::program::ExampleHandlers, ExampleHandlerClient);

    impl ExampleHandlerClient {

        pub fn run_test() -> Result<()> {

            Ok(())
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

