<p align="center">
  <a href="https://travis-ci.org/jeandudey/bitcoin-rpc.svg?branch=master" alt="Build Status">
  </a>

  <a href="https://crates.io/crates/bitcoin-rpc">
    <img src="https://img.shields.io/crates/v/bitcoin-rpc.svg?maxAge=2592000"></img>
  </a>

  <br/>

   <strong>
     <a href="https://jeandudey.github.io/jsonrpc-v1">
       Documentation
     </a>
   </strong>
</p>

# Bitcoin RPC
This crate implements an Bitcoin RPC client in rust, this cate doesn't intends to be a complete implementation of all the bitcoin rpc methods so if you need some method you can create a pull request for it.

## AltCoins
I don't have tested it for other cryptocurrencies, only bitcoin was tested.

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
bitcoin-rpc = "0.1"
```

And this to your crate root:
```rust
extern crate bitcoin_rpc;
```

### Example: Connecting to bitcoin rpc server
```rust
extern crate bitcoin_rpc;

use bitcoin_rpc::BitcoinRpc;

fn main() {
    let client = BitcoinRpc::new("example.org:8331", None, None);

    let block_count = match client.getblockcount() {
        Ok(b) => b,
        Err(e) => panic!("error: {}", e);
    }

    println!("Block count: {}", block_count);
}
```
