use std::sync::Arc;
use axum::{
    extract,
    Extension,
};

use crate::ServerState;

fn join_testnet_thread(server_state: &Arc<ServerState>, rpc_url: &String) -> Result<String, String> {
    let mut testnets_map = server_state.testnets.write().unwrap();
    testnets_map.remove(rpc_url)
        .ok_or(String::from("No testnet with this RPC URL"))
        .and_then(
            |(ref mut testnet_process, _)| {
                testnet_process.kill()
                    .map_err(
                        |err| format!("Testnet at RPC URL {} already exited: {}", rpc_url, err),
                    )?;

                // Even though the process has terminated, wait on it to prevent zombification.
                testnet_process.wait()
                    .map_or_else(
                        |err| Err(format!("Error waiting on testnet at RPC URL {}: {}", rpc_url, err)),
                        |ok| Ok(format!("Testnet at RPC URL {} exited with status {}", rpc_url, ok))
                    )
            }
        )
}

pub async fn destroy(
    Extension(server_state): Extension<Arc<ServerState>>,
    extract::Json(rpc_url): extract::Json<String>,
) -> String {
    match join_testnet_thread(&server_state, &rpc_url) {
        Ok(ok) => ok,
        Err(err) => err,
    }
}