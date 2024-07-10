use std::{env, sync::Arc};

use tokio::time;
use zenoh::{prelude::*, Config, Session};

const KEY_EXPR: &str = "slow_queryable";
const VALUE: &str = "🐢";

async fn declare_queryable(session: Arc<Session>, i: usize) {
    let queryable = session.declare_queryable(KEY_EXPR).await.unwrap();

    // {
    //     let queryable = queryable.clone();
    //     task::spawn(async move {
    //         let mut previous = queryable.len();
    //         loop {
    //             let current = queryable.len();
    //             if current != previous {
    //                 previous = current;
    //                 eprintln!("slow_queryable: {i}: channel-len: {}", current);
    //             }
    //         }
    //     });
    // }

    while let Ok(query) = queryable.recv_async().await {
        time::sleep(time::Duration::from_secs(1)).await;
        query.reply(KEY_EXPR, VALUE).await.unwrap();
        eprintln!(
            "slow_queryable: {i}: replied: {} (channel-len: {})",
            query,
            queryable.len()
        );
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter("zenoh::instrumentation=trace")
        .with_target(true)
        .init();

    let config_path = env::args().nth(1).unwrap();
    let config = Config::from_file(config_path).unwrap();
    let session = Arc::new(zenoh::open(config).await.unwrap());

    let mut handles = Vec::new();
    for i in 0..2 {
        handles.push(
            zenoh_runtime::ZRuntime::Application.spawn(declare_queryable(session.clone(), i)),
        );
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
