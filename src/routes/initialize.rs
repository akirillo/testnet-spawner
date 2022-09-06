use std::{
    process::{
        Command,
        Stdio,
        Child,
        ChildStdout,
    },
    io::{
        BufRead,
        BufReader,
    },
    sync::Arc,
    env,
    fmt,
};
use axum::{
    extract,
    response,
    Extension,
};
use serde::{
    Deserialize,
    Serialize
};
use serde_json::{
    Value,
    json,
};

use crate::ServerState;
// use crate::testnet::pool::{
//     TESTNETS,
//     PORT,
// };

#[derive(Deserialize)]
pub enum TestnetStateType {
    Default,
    Mainnet,
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

async fn get_rpc_endpoint(server_state: &Arc<ServerState>) -> GetRPCResult<(String, String, String)> {
    let rpc_host = String::from("127.0.0.1");
    let mut port = server_state.port.lock().unwrap();
    let rpc_port = port.to_string();
    *port += 1;
    let rpc_url = format!("{}:{}", rpc_host, rpc_port);
    Ok((
        rpc_host,
        rpc_port,
        rpc_url
    ))
}

#[derive(Debug, Serialize)]
struct DevAccount {
    pub_key: String,
    priv_key: String,
}

fn parse_dev_accounts(stdout: &mut ChildStdout, num_accounts: usize) -> Vec<DevAccount> {
    let mut dev_accounts: Vec<DevAccount> = Vec::new();

    let lines: Vec<String> = BufReader::new(stdout).lines()
        .map(|line| line.unwrap())
        .take_while(
            |line| !line.contains("Listening")
        ).collect();

    let sections: Vec<&[String]> = lines.split(|line| line.contains("==================")).collect();
    let pub_keys_section = &sections[1][1..=num_accounts];
    let priv_keys_section = &sections[2][1..=num_accounts];

    let pub_keys = pub_keys_section.iter().map(
        |pub_key| String::from(pub_key.split(" ").nth(1).expect("Malformed pub key line"))
    );

    let priv_keys = priv_keys_section.iter().map(
        |priv_key| String::from(priv_key.split(" ").nth(1).expect("Malformed priv key line"))
    );

    let keypairs = pub_keys.zip(priv_keys);

    for (pub_key, priv_key) in keypairs {
        dev_accounts.push(DevAccount {
            pub_key: pub_key,
            priv_key: priv_key,
        })
    }

    dev_accounts
}

async fn store_testnet_process(server_state: &Arc<ServerState>, testnet_process: Child, rpc_url: String) {
    let mut testnets_map = server_state.testnets.write().unwrap();
    let testnet_process_id = testnet_process.id();
    testnets_map.insert(rpc_url.clone(), (testnet_process, testnet_process_id));
}

// TODO: More graceful error handling
pub async fn initialize(
    Extension(server_state): Extension<Arc<ServerState>>,
    extract::Json(testnet_state_type): extract::Json<TestnetStateType>
) -> response::Json<Value> {
    // STUB: May be async later on (e.g. provision infra, then get host/port)
    let (rpc_host, rpc_port, rpc_url) = get_rpc_endpoint(&server_state).await.expect("Error getting RPC endpoint");
    let mut args: Vec<String> = vec![
        String::from("--host"),
        rpc_host,
        String::from("--port"),
        rpc_port,
        String::from("--block-time"),
        String::from("12"),
    ];

    match testnet_state_type {
        TestnetStateType::Default => {},
        TestnetStateType::Mainnet => {
            args.append(&mut vec![
                String::from("--fork-url"),
                env::var("MAINNET_RPC_URL").expect("MAINNET_RPC_URL env var not set")
            ]);
        },
    };

    let mut testnet_process = Command::new("anvil")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .args(args)
        .spawn()
        .expect("Error forking testnet process");

    let mut stdout = testnet_process.stdout.take().expect("Error piping stdout");
    let dev_accounts = parse_dev_accounts(&mut stdout, 10);

    // STUB: May be async later on (e.g. store metadata in a DB)
    store_testnet_process(&server_state, testnet_process, rpc_url.clone()).await;

    response::Json(json!({
        "rpc_url": rpc_url,
        "dev_accounts": dev_accounts
    }))
}
