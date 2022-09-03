use std::process::Command;
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

async fn get_rpc_host() -> GetRPCResult<String> {
    Ok("127.0.0.1".into())
}

async fn get_rpc_port() -> GetRPCResult<String> {
    Ok("8545".into())
}

pub async fn initialize(Json(state_type): Json<InitializeStateType>) -> String {
    match state_type {
        InitializeStateType::Default => {            
            // Stub, later on this may actually need to be async (e.g. fetch host:port from load balancer, host/port pools)
            let rpc_host = match get_rpc_host().await {
                Ok(rpc_host) => rpc_host,
                Err(err) => return format!("Error getting RPC host: {}", err),
            };
            let rpc_port = match get_rpc_port().await {
                Ok(rpc_port) => rpc_port,
                Err(err) => return format!("Error getting RPC port: {}", err),
            };
            let rpc_url = format!("{}:{}", rpc_host, rpc_port);

            // TODO: Handle w/o unwrap
            // TODO: Handling of stdin/stdout/stderr
            // TODO: If args are invalid, we should error in the main process, as well
            // TODO: Include anvil binary in repo instead of assuming it's present?
            //       ^ TBF, this should prob look like Dockerizing this service and `foundryup`-ing in the Dockerfile
            let testnet_process = Command::new("anvil")
                .arg("--host")
                .arg(rpc_host)
                .arg("--port")
                .arg(rpc_port)
                .spawn()
                .unwrap();

            // Store sender endpoint in hashmap
            let mut testnets_map = TESTNETS.write().unwrap();
            let testnet_process_id = testnet_process.id();
            testnets_map.insert(rpc_url.clone(), (testnet_process, testnet_process_id));
            format!("RPC URL: {}, PID: {}", rpc_url, testnet_process_id)
        },
        // InitializeStateType::Mainnet => {
        //     let mut testnets_map = TESTNETS.lock().unwrap();
        //     testnets_map.insert(0, "rpcurl".into());
        //     let url_clone = testnets_map.get(&0).clone();
        //     url_clone.map(|&s| s)
        // },
    }
}
