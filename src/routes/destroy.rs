use std::sync::Arc;
use axum::{
    extract,
    Extension,
};

use crate::ServerState;

fn join_testnet_thread(server_state: &Arc<ServerState>, rpc_url: &String) -> Result<String, String> {
    let mut testnets_map = server_state.testnets.write().unwrap();
    testnets_map.remove(rpc_url)
        // Transform Option<JoinHandle<String>> into Result<JoinHandle<String>, String>
        .ok_or(String::from("No testnet with this RPC URL."))
        // Propagate Err(String) or call func on Ok(JoinHandle<String>) that returns Result<String, String>
        .and_then(
            |(ref mut testnet_process, testnet_process_id)| {
                testnet_process.kill()
                    .map_err(
                        |err| format!("Testnet process {} already exited: {}", testnet_process_id, err),
                    )?;

                // Even though the process has terminated, wait on it to prevent zombification.
                testnet_process.wait()
                    .map_or_else(
                        |err| Err(format!("Error waiting on testnet process {}: {}", testnet_process_id, err)),
                        |ok| Ok(format!("Testnet process {} at RPC URL {} exited with status {}", testnet_process_id, rpc_url, ok))
                    )
            }
        )
}

pub async fn destroy(
    Extension(server_state): Extension<Arc<ServerState>>,
    extract::Json(rpc_url): extract::Json<String>,
) -> String {
    // Is there a cleaner / idiomatic way to do this?
    match join_testnet_thread(&server_state, &rpc_url) {
        Ok(result) => result,
        Err(err) => err,
    }
}