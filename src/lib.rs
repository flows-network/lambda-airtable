use std::{collections::HashMap, env};

use airtable_flows::{create_record, update_record};
use dotenv::dotenv;
use flowsnet_platform_sdk::logger;
use lambda_flows::{request_received, send_response};

use serde_json::Value;

type Response = (u16, Vec<(String, String)>, Vec<u8>);

struct Record<'a> {
    account: &'a str,
    base_id: &'a str,
    table_name: &'a str,
}

impl Record<'_> {
    fn create(&self, text: Value) {
        create_record(self.account, self.base_id, self.table_name, text);
    }
    fn update(&self, record_id: &str, text: Value) {
        update_record(self.account, self.base_id, self.table_name, record_id, text);
    }
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    logger::init();

    let account = env::var("account").unwrap();

    request_received(|qry, body| handler(qry, body, account)).await;
}

async fn handler(qry: HashMap<String, Value>, body: Vec<u8>, account: String) {
    let action = qry
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("create");

    let res = || -> Result<(&str, &str), Response> {
        let base_id = extract_query(&qry, "base_id")?;
        let table_name = extract_query(&qry, "table_name")?;

        Ok((base_id, table_name))
    }();

    let (base_id, table_name) = match res {
        Ok(r) => r,
        Err((status, headers, body)) => {
            send_response(status, headers, body);
            return;
        }
    };

    let record = Record {
        account: &account,
        base_id,
        table_name,
    };

    let text = match serde_json::from_slice(&body) {
        Ok(t) => t,
        Err(e) => {
            log::debug!("error occurs when deserialize body: {}", e);
            let error = "body must be a valid json".to_string();
            send_response(400, vec![], error.into_bytes());
            return;
        }
    };

    match action {
        "create" => {
            record.create(text);
        }
        "update" => {
            let record_id = extract_query(&qry, "record_id");
            match record_id {
                Ok(rid) => {
                    record.update(rid, text);
                }
                Err((status, headers, body)) => {
                    log::debug!("no record_id is provided");
                    send_response(status, headers, body);
                }
            }
        }
        act => {
            let error = format!("invalid action: {}, valid actions: `create`, `update`", act);
            log::info!("invalid action: {}", act);
            send_response(400, vec![], error.into_bytes());
        }
    }
}

fn extract_query<'a>(qry: &'a HashMap<String, Value>, key: &'a str) -> Result<&'a str, Response> {
    let value = qry.get(key);

    match value {
        Some(v) => {
            let vs = v.as_str();
            match vs {
                Some(s) => Ok(s),
                None => {
                    let error = format!("`{}` should be valid String", key);
                    log::debug!("{} not a valid String serde_json::Value", key);
                    Err((400, vec![], error.into_bytes()))
                }
            }
        }
        None => {
            let error = format!("missing key `{}` in request query", key);
            log::debug!("{} not in query HashMap", key);
            Err((400, vec![], error.into_bytes()))
        }
    }
}
