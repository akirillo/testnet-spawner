use std::process::{
    Command,
    Stdio,
    ChildStdout
};
use std::io::{
    BufRead,
    BufReader,
};
use std::fmt;
use axum::{
    extract,
    response
};
use serde::{
    Deserialize,
    Serialize
};
use serde_json::{Value, json};

use crate::testnet::pool::TESTNETS;

#[derive(Deserialize)]
pub enum InitializeStateType {
    Default,
    // Mainnet,
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

async fn get_rpc_host() -> GetRPCResult<String> {
    Ok("127.0.0.1".into())
}

async fn get_rpc_port() -> GetRPCResult<String> {
    Ok("8545".into())
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

pub async fn initialize(extract::Json(state_type): extract::Json<InitializeStateType>) -> response::Json<Value> {
    match state_type {
        InitializeStateType::Default => {            
            // Stub, later on this may actually need to be async (e.g. fetch host:port from load balancer, host/port pools)
            let rpc_host = get_rpc_host().await.expect("Error getting RPC host");
            let rpc_port = get_rpc_port().await.expect("Error getting RPC port");
            let rpc_url = format!("{}:{}", rpc_host, rpc_port);

            // TODO: Handling of stdin/stdout/stderr
            // TODO: Parse dev accounts from stdout
            let mut testnet_process = Command::new("anvil")
                .stdin(Stdio::null())
                // TODO: If args are invalid, we should error in the main process, as well
                // TODO: Include anvil binary in repo instead of assuming it's present?
                //       ^ TBF, this should prob look like Dockerizing this service and `foundryup`-ing in the Dockerfile
                .stderr(Stdio::null())
                .stdout(Stdio::piped())
                .arg("--host")
                .arg(rpc_host)
                .arg("--port")
                .arg(rpc_port)
                .spawn()
                .expect("Error forking testnet process");
            
            let mut stdout = testnet_process.stdout.take().expect("Error piping stdout");
            let dev_accounts = parse_dev_accounts(&mut stdout, 10);

            let mut testnets_map = TESTNETS.write().unwrap();
            let testnet_process_id = testnet_process.id();
            testnets_map.insert(rpc_url.clone(), (testnet_process, testnet_process_id));

            response::Json(json!({
                "rpc_url": rpc_url,
                "PID": testnet_process_id,
                "dev_accounts": dev_accounts
            }))
        },
        // InitializeStateType::Mainnet => {
        //     let mut testnets_map = TESTNETS.lock().unwrap();
        //     testnets_map.insert(0, "rpcurl".into());
        //     let url_clone = testnets_map.get(&0).clone();
        //     url_clone.map(|&s| s)
        // },
    }
}
