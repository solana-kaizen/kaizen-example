# workflow-allocator-example

This is a simple example demonstrating the Solana [Workflow Allocator](https://github.com/workflow-rs/workflow-allocator) framework.

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

The dependencies of Workflow Allocator are not currently published to crates.io or are
out of date.  To setup a development environment, you need to clone its dependencies
manually or use [Emanator](https://github.com/aspectron/cargo-emanate)

This will be addressed in the coming weeks.

### Deploying via Emanator

To deploy this example using Emanator, you need to have a functional command-line `git` and run the following:

```
cargo install cargo-emanate
git clone https://github.com/workflow-rs/workflow-allocator-example-dev
cd workflow-allocator-example-dev
cargo emanate sync
cd workflow-allocator-example-dev/workflow-allocator-example
```

### Deploying manually:

```
mkdir example
cd example
git clone https://github.com/workflow-rs/workflow-macro-tools
git clone https://github.com/workflow-rs/workflow-websocket
git clone https://github.com/workflow-rs/workflow-rpc
git clone https://github.com/workflow-rs/workflow-core
git clone https://github.com/workflow-rs/workflow-log
git clone https://github.com/workflow-rs/workflow-wasm
git clone https://github.com/workflow-rs/workflow-panic-hook
git clone https://github.com/workflow-rs/workflow-async-trait
git clone https://github.com/workflow-rs/workflow-allocator
git clone https://github.com/workflow-rs/workflow-allocator-example
cd workflow-allocator-example
```

### Running

Following this you can build subfolders as follows:
* `native` - `cargo run`
* `wasm` - `./build`
* `program` - `cargo build-bpf`
* `simulator` - `cargo run`

To access WASM target, you can use any web server from the `root` folder.

For example:
```
cargo install simple-http-server
cd root
simple-http-server
```
Following that, access http://localhost:8000/index.html and open the browser developer console to see application output logs.
