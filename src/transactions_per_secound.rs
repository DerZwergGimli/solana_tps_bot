use std::env;
use std::fmt::format;

use log::{error, warn};
use solana_client::rpc_client::RpcClient;
use solana_sdk::clock::Slot;
use solana_sdk::commitment_config::CommitmentConfig;

//region private
fn connect_to_rpc() -> RpcClient {
    let rpc_url = String::from(env::var("RPC_ENDPOINT").expect("unable to find env 'RPC_ENDPOINT'"));
    RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed())
}

fn get_and_calc_tps(client: RpcClient) -> i64 {
    let client = connect_to_rpc();
    let last_block_height = client.get_block_height().unwrap_or(0);

    let blocks = client.get_blocks_with_limit(last_block_height, 20).unwrap_or(Vec::new());

    if !blocks.is_empty() {
        let mut transactions_count = 0;
        for block in blocks.clone() {
            match client.get_block(block) {
                Ok(data) => {
                    transactions_count += data.transactions.len() as i64;
                }
                Err(_) => { warn!("unable to fetch block details") }
            }
        }

        let time_block_first = match client.get_block(*blocks.clone().first().unwrap()) {
            Ok(data) => { data.block_time.unwrap_or_default() }
            Err(_) => {
                warn!("unable to fetch block details");
                0
            }
        };


        let time_block_last = match client.get_block(*blocks.clone().last().unwrap()) {
            Ok(data) => { data.block_time.unwrap_or_default() }
            Err(_) => {
                warn!("unable to fetch block details");
                0
            }
        };

        transactions_count / (time_block_last as i64 - time_block_first as i64)
    } else {
        error!("unable to get TPS");
        0
    }
}

//endregion


//region public

pub fn get_tps() -> i64 {
    get_and_calc_tps(connect_to_rpc())
}

pub fn get_tps_string(tps: i64) -> String {
    let threshold = env::var("TPS_THRESHOLD").unwrap_or("2000".to_string()).parse::<i64>().unwrap();
    let mut text = "".to_string();
    if tps > threshold {
        text = format!("🚀 ~{:?} TPS", tps)
    } else {
        text = format!("🔥 ~{:?} TPS", tps)
    }
    text
}

//endregion

