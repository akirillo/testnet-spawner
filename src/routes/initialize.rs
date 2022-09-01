use std::thread;
use std::fmt;
use axum::{
    extract::Json,
};
use serde::Deserialize;

use crate::testnet::pool::TESTNETS;

#[derive(Deserialize)]
pub enum InitializeStateType {
    Default,
    // Mainnet,
    // Snapshot, (TODO)
}

#[derive(Debug, Clone)]
struct GetRPCError;

impl fmt::Display for GetRPCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to get and RPC URL.")
    }
}

type GetRPCResult<T> = Result<T, GetRPCError>;

async fn get_rpc_url() -> GetRPCResult<String> {
    Ok("rpcurl".into())
}

pub async fn initialize(Json(state_type): Json<InitializeStateType>) -> String {
    match state_type {
        InitializeStateType::Default => {
            // Spawn a thread, store its handle in a (for now, in-memory) mapping
            let rpc_url = match get_rpc_url().await {
                Ok(rpc_url) => rpc_url,
                Err(err) => format!("{}", err),
            };
            let testnet_thread = thread::spawn(move || {
                String::from("testnet!")
            });
            let mut testnets_map = TESTNETS.write().unwrap();
            testnets_map.insert(rpc_url.clone(), testnet_thread);
            rpc_url
        },
        // InitializeStateType::Mainnet => {
        //     let mut testnets_map = TESTNETS.lock().unwrap();
        //     testnets_map.insert(0, "rpcurl".into());
        //     let url_clone = testnets_map.get(&0).clone();
        //     url_clone.map(|&s| s)
        // },
    }
}
