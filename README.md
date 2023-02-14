# Kaizen Example

This is a simple example demonstrating the [Solana Kaizen](https://github.com/solana-kaizen/kaizen) framework.

This example contains the following folders:

* `lib` - solana program + client-side functions for program access
* `native` - binary target for the `lib` crate
* `wasm` - WASM target for the `lib` crate
* `root` - contains `index.html` that loads the wasm library
* `simulator` - emulator that embeds the program in the `lib` crate (accessible from `lib` - i.e. `native` or `wasm` target)
* `program` - crate for building lib program (you can build multiple programs from `lib` but then `declare_program!()` macro has to be declared in each program respectively)


## Prerequisites:

Rust - https://www.rust-lang.org/

To install:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Wasm Pack - https://rustwasm.github.io/wasm-pack/

To install:
```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Solana Tool Suite - https://docs.solana.com/cli/install-solana-cli-tools

To install:
```
sh -c "$(curl -sSfL https://release.solana.com/v1.14.7/install)"
```

## Dependencies

The project is not currently published on crates.io (it will be in the coming weeks) as such, to build it
you need to clone the following repositories in the same folder:
- https://github.com/solana-kaizen/kaizen
- https://github.com/solana-kaizen/solana-web3-sys
- https://github.com/workflow-rs/workflow-rs

### Running

Following this you can build subfolders as follows:
* `native` (native client execution) - `cargo run`
* `wasm` (web-browser client; see below) - `./build`
* `program` (program for solana deployment) - `cargo build-sbf`
* `simulator` (simulator server) - `cargo run`
* `lib` (unit tests) - `cargo test example_test -- --nocapture`

NOTE: `program` also has a `deploy` script that uses the included demo key that produces a matching program id.

To access WASM target, you can use any web server from the `root` folder.

For example:
```
cargo install basic-http-server
cd root
basic-http-server
```
Following that, access http://localhost:4000/index.html and open the browser developer console to see application output logs.

Please note: Current implementation of the example, does not connect to the in-browser wallet, such as phantom
as this example was quickly put together to demonstrate the general functionality of the framework, it does not have 
any user interface.  The example of interfacing with wallet adapters will be done at a later date.
