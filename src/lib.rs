use lambda_flows::{request_received, send_response};
use flowsnet_platform_sdk::logger;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    request_received(|headers, qry, body| {
        login(headers, qry, body)
    }).await;
    Ok(())
}

async fn login(headers: Vec<(String, String)>, _qry: HashMap<String, Value>, body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    let login_info: LoginPayload = match serde_json::from_slice(&body) {
        Ok(info) => info,
        Err(err) => {
            log::error!("Failed to parse login JSON: {}", err);
            send_response(400, vec![], b"Bad Request".to_vec());
            return;
        }
    };

    if login_info.username == "admin" && login_info.password == "admin" {
        send_response(200, vec![], b"Login Success\n".to_vec());
    } else {
        send_response(401, vec![], b"Login Failed\n".to_vec());
    }
}
