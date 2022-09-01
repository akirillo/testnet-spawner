mod routes;
mod testnet;

// use std::sync::{Arc, RwLock, Mutex};
// use std::thread::JoinHandle;
// use std::collections::HashMap;
use axum::{
    routing::post,
    Router,
};
use routes::{
    initialize::initialize,
    destroy::destroy,
};


#[tokio::main]
async fn main() {
    // let testnets_map: Arc<RwLock<HashMap<String, Mutex<JoinHandle<String>>>>> = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/initialize", post(initialize))
        // .route("/reset", post())
        .route("/destroy", post(destroy));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
