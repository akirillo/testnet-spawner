use std::sync::Arc;
use axum::{
    extract,
    Extension
};
use axum_macros::debug_handler;
use jsonrpsee::{    
    http_client::{HttpClientBuilder, HttpClient},
    core::client::ClientT,
    rpc_params,
};

use crate::ServerState;

async fn reset_testnet_state(server_state: &Arc<ServerState>, rpc_url: &String) -> Result<String, String> {
    let http_client: HttpClient;
    let snapshot_id: String;
    {
        let testnets_map = server_state.testnets.read()
            .map_err(|_| String::from("RwLock poisoned"))?;
    
        (http_client, snapshot_id) = testnets_map.get(rpc_url)
            .ok_or(String::from("No testnet with this RPC URL"))
            .and_then(
                |(_, snapshot_id)| {
                    // Turn this into Result<(HttpClient, i32), String>
                    HttpClientBuilder::default().build(format!("http://{}", rpc_url))
                        .map(
                            |http_client| (http_client, snapshot_id.clone())
                        )
                        .map_err(
                            |err| format!("Error building RPC client: {}", err),
                        )
                }
            )?;
    }
    
    http_client.request("evm_revert", rpc_params![snapshot_id]).await
        .map_err(|err| format!("Error sending `evm_revert` RPC request: {}", err))
}

#[debug_handler]
pub async fn reset(
    Extension(server_state): Extension<Arc<ServerState>>,
    extract::Json(rpc_url): extract::Json<String>,
) -> String {
    match reset_testnet_state(&server_state, &rpc_url).await {
        Ok(ok) => ok,
        Err(err) => err,
    }
}