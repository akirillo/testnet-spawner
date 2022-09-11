mod routes;

use std::sync::{
    RwLock,
    Mutex,
    Arc,
};
use std::process::Child;
use std::collections::HashMap;
use axum::{
    routing::post,
    Router,
    Extension,
};
use routes::{
    initialize::initialize,
    destroy::destroy,
    reset::reset,
};

#[allow(dead_code)]
pub struct ServerState {
    testnets: RwLock<HashMap<String, (Child, String)>>,
    port: Mutex<u32>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(ServerState {
        testnets: RwLock::new(HashMap::new()),
        port: Mutex::new(8545),
    });

    let app = Router::new()
        .route("/initialize", post(initialize))
        .route("/reset", post(reset))
        .route("/destroy", post(destroy))
        .layer(Extension(state));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
