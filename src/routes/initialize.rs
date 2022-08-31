use axum::{
    extract::Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub enum InitializeStateType {
    Default,
    Mainnet,
    // Snapshot, (TODO)
}

pub async fn initialize(Json(state_type): Json<InitializeStateType>) -> String {
    match state_type {
        InitializeStateType::Default => {
            // Spawn a thread, store its handle in a (for now, in-memory) mapping
            "rpcurl".into()
        },
        InitializeStateType::Mainnet => "rpcurl".into(),
    }
}
