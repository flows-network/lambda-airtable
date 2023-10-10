use serde_json::Value;
use std::collections::HashMap;

use webhook_flows::{create_endpoint, request_handler, send_response};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler]
async fn handler(
    _headers: Vec<(String, String)>,
    _subpath: String,
    qry: HashMap<String, Value>,
    body: Vec<u8>,
) {
    let html = include_str!("index.html");
    let backurl = std::env::var("BACKEND_SERVICE_URL").expect("No url in env var");
    let base_id = std::env::var("BASE_ID").expect("No base_id in env var");
    let table_name = std::env::var("TABLE_NAME").expect("No table_name in env var");
    let html = html
        .replace("{BACKEND_SERVICE_URL}", &backurl)
        .replace("{BASE_ID}", &base_id)
        .replace("{TABLE_NAME}", &table_name);
    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        html.as_bytes().to_vec(),
    );
}
