#[macro_use]
extern crate jsonrpc_v1;
extern crate strason;
extern crate serde;

use jsonrpc_v1::client::Client as RpcClient;
use jsonrpc_v1::Error as RpcError;
use strason::Json;


/// A Handle to a Bitcoin Rpc connection
pub struct BitcoinRpc {
    client: RpcClient,
}

pub struct SerializedBlock {
    pub result: String,
}

pub struct Block {
    pub hash: String,
    pub confirmations: i64,
    pub size: i64,
    pub height: i64,
    pub version: i64,
    pub merkleroot: String,
    pub tx: Vec<Json>,
    pub txid: String,
    pub time: i64,
    pub nonce: i64,
    pub bits: String,
    pub chainwork: String,
    pub previousblockhash: Option<String>,
    pub nextblockhash: Option<String>,
}

pub enum GetBlockReply {
    True(Block),
    False(SerializedBlock),
}

serde_struct_enum_impl!(GetBlockReply,
                        True, Block, hash, confirmations, size, height, version, merkleroot, tx, txid <- "TXID", time, nonce, bits,  chainwork, previousblockhash, nextblockhash;
                        False, SerializedBlock, result
);

impl BitcoinRpc {
    /// Creates a connection to a bitcoin rpc server
    pub fn new(url: &str, user: Option<String>, pass: Option<String>) -> Self {
        // Check that if we have a password, we have a username; other way around is ok
        debug_assert!(pass.is_none() || user.is_some());

        BitcoinRpc {
            client: RpcClient::new(String::from(url), user, pass),
        }
    }

    pub fn getbestblockhash(&self) -> Result<String, RpcError> {
        let request = self.client.build_request(String::from("getbestblockhash"), vec![]);

        match self.client.send_request(&request).and_then(|res| res.into_result::<String>()) {
            Ok(blockhash) => return Ok(blockhash),
            Err(e) => return Err(e),
        }
    }


    pub fn getblock(&self, header_hash: String, format: bool) -> Result<GetBlockReply, RpcError> {
        let params: Vec<Json> = vec![Json::from(header_hash), Json::from(format)];
        let request = self.client.build_request(String::from("getblock"), params);

        match self.client.send_request(&request).and_then(|res| res.into_result::<GetBlockReply>()) {
            Ok(reply) => return Ok(reply),
            Err(e) => return Err(e),
        }
    }
}

