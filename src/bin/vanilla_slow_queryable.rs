use std::{env, time::Duration};

use zenoh::{prelude::*, Config};

const KEY_EXPR: &str = "slow_queryable";
const VALUE: &str = "🐢";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter("zenoh::instrumentation=trace")
        .with_level(false)
        .init();

    let config_path = env::args().nth(1).unwrap();
    let interval = env::args().nth(2).unwrap().parse::<u64>().unwrap();
    let config = Config::from_file(config_path).unwrap();
    let session = zenoh::open(config).await.unwrap();

    let queryable = session.declare_queryable(KEY_EXPR).await.unwrap();

    while let Ok(query) = queryable.recv_async().await {
        tokio::time::sleep(Duration::from_millis(interval)).await;
        query.reply(KEY_EXPR, VALUE).await.unwrap();
    }
}
