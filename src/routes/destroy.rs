use axum::{
    extract::Json,
};

use crate::testnet::pool::TESTNETS;

fn join_testnet_thread(rpc_url: &String) -> Result<String, String> {
    // Next step: write to channel that causes thread to close.
    let mut testnets_map = TESTNETS.write().unwrap();
    testnets_map.remove(rpc_url)
        // Transform Option<JoinHandle<String>> into Result<JoinHandle<String>, String>
        .ok_or(String::from("No testnet with this RPC URL."))
        // Propagate Err(String) or call func on Ok(JoinHandle<String>) that returns Result<String, String>
        .and_then(
            |join_handle| join_handle.join()
            // Transform Result<String, Box<dyn Any + Send + 'static>> into Result<String, String>
            .map_err(|err| format!("Testnet thread panicked: {:?}", err))
        )
}

pub async fn destroy(Json(rpc_url): Json<String>) -> String {
    // Is there a cleaner / idiomatic way to do this?
    match join_testnet_thread(&rpc_url) {
        Ok(result) => result,
        Err(err) => err,
    }
}