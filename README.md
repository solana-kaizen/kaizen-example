# workflow-allocator-example

This is a simple example demonstrating the Solana [Workflow Allocator](https://github.com/workflow-rs/workflow-allocator) framework.

This example contains the following folders:

* `lib` - solana program + client-side functions for program access
* `native` - binary target for the `lib` crate
* `wasm` - WASM target for the `lib` crate
* `root` - contains `index.html` that loads the wasm library
* `simulator` - emulator that embeds the program in the `lib` crate (accessible from `lib` - i.e. `native` or `wasm` target)
* `program` - crate for building lib program (you can build multiple programs from `lib` but then `declare_program!()` macro has to be declared in each program respectively)


