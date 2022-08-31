mod routes;

use axum::{
    routing::post,
    Router,
};
use routes::initialize::initialize;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/initialize", post(initialize));
        // .route("/reset", post())
        // .route("/destroy", post());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
