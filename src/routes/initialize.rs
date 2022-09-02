use std::thread;
use std::sync::{
    Mutex,
    mpsc::{
        channel,
        TryRecvError,
    }
};
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
            // Spawn a thread, create a channel, store the sender endpoint in a global in-memory hashmap.
            
            // Stub, later on this may actually need to be async (i.e. deploy a bunch of nodes, set up load-balancing RPC endpoint, return it)
            let rpc_url = match get_rpc_url().await {
                Ok(rpc_url) => rpc_url,
                Err(err) => format!("{}", err),
            };

            // Create channel that will be used to send the thread a termination message.
            let (tx, rx) = channel();
            
            // Should I use tokio::spawn here instead?
            // I figure that since these are meant to be persistent processes (testnets),
            // they should be on dedicated (native) threads, and not as async tasks on one thread.
            let testnet_thread = thread::spawn(move || {
                let mut count = 0;
                let result;
                loop {
                    match rx.try_recv() {
                        Ok(_) | Err(TryRecvError::Disconnected) => {
                            result = format!("Terminated at count: {}", count);
                            break;
                        }
                        Err(TryRecvError::Empty) => {}
                    }
                    count += 1;
                }
                result
            });

            // Store sender endpoint in hashmap
            let mut testnets_map = TESTNETS.write().unwrap();
            testnets_map.insert(rpc_url.clone(), (Mutex::new(tx), testnet_thread));
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
