![Cargo Checks](https://github.com/GalactechsLLC/dg_xch_utils/actions/workflows/rust.yml/badge.svg)

FastFarmer
=====

A lite farmer for the Chia Blockchain.


Building
--------

Install Rust by following the instructions at https://www.rust-lang.org/tools/install

Once Rust is installed we can build from source:
```
git clone https://github.com/GalactechsLLC/dg_fast_farmer.git
cd dg_fast_farmer
cargo build --release
sudo cp target/release/ff /usr/local/bin/ff
```

Running
--------

To generate the farmer config:
```
ff init -m "MNEMONIC" -f FULLNODE_HOST -p FULLNODE_PORT -n SELECTED_NETWORK
```

To use a separate Fullnode for RPC calls during setup:
```
ff init -m "MNEMONIC" -f FULLNODE_HOST -p FULLNODE_PORT -r FULLNODE_RPC_HOST -o FULLNODE_RPC_PORT -n SELECTED_NETWORK
```

To run the Farmer with TUI Interface(Default):
```
ff
```

To run the Farmer in CLI mode:
```
ff run
```
