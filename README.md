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

IMPORTANT:

The dependencies of Solana Kaizen are not currently published to crates.io or are
out of date.  To setup a development environment, you need to clone its dependencies
manually or use [Emanator](https://github.com/aspectron/cargo-emanate)

This will be addressed in the coming weeks.

### Deploying via Emanator

To deploy this example using Emanator, you need to have a functional command-line `git` and run the following:

```
cargo install cargo-emanate
git clone https://github.com/solana-kaizen/solana-kaizen-example-dev
cd solana-kaizen-example-dev
cargo emanate sync
cd solana-kaizen-example-dev/solana-kaizen-example
```

### Running

Following this you can build subfolders as follows:
* `native` (native client execution) - `cargo run`
* `wasm` (web-browser client; see below) - `./build`
* `program` (program for solana deployment) - `cargo build-bpf`
* `simulator` (simulator server) - `cargo run`
* `lib` (unit tests) - `cargo test example_test -- --nocapture`

To access WASM target, you can use any web server from the `root` folder.

For example:
```
cargo install simple-http-server
cd root
simple-http-server
```
Following that, access http://localhost:8000/index.html and open the browser developer console to see application output logs.

Please note: Current implementation of the example, does not connect to the in-browser wallet, such as phantom
as this example was quickly put together to demonstrate the functionality of the framework, it does not have 
any user interface.  The example of interfacing with wallet adapters will be done at a later date.

