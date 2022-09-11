use std::sync::Arc;
use axum::{
    extract,
    Extension,
};

use crate::ServerState;

fn join_testnet_thread(server_state: &Arc<ServerState>, rpc_url: &String) -> Result<String, String> {
    // Can I use and_then here instead of unwrapping?
    let mut testnets_map = server_state.testnets.write().unwrap();
    testnets_map.remove(rpc_url)
        // Transform Option<JoinHandle<String>> into Result<JoinHandle<String>, String>
        .ok_or(String::from("No testnet with this RPC URL"))
        // Propagate Err(String) or call func on Ok(JoinHandle<String>) that returns Result<String, String>
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
    // Is there a cleaner / idiomatic way to do this?
    // ^ kinda: Result<T, T>.into_ok_or_err, but this is experimental
    match join_testnet_thread(&server_state, &rpc_url) {
        Ok(ok) => ok,
        Err(err) => err,
    }
}