use std::{env, sync::Arc};

use tokio::{task, time};
use zenoh::prelude::r#async::*;

const KEY_EXPR: &str = "slow_queryable";
const VALUE: &str = "🐢";

async fn declare_queryable(session: Arc<Session>, i: usize) {
    let queryable = session.declare_queryable(KEY_EXPR).res().await.unwrap();

    {
        let queryable = queryable.clone();
        task::spawn(async move {
            let mut previous = queryable.len();
            loop {
                let current = queryable.len();
                if current != previous {
                    previous = current;
                    eprintln!("slow_queryable: {i}: channel-len: {}", current);
                }
            }
        });
    }

    while let Ok(query) = queryable.recv_async().await {
        let reply = Ok(Sample::try_from(KEY_EXPR, VALUE).unwrap());
        time::sleep(time::Duration::from_secs(1)).await;
        query.reply(reply).res().await.unwrap();
        eprintln!("slow_queryable: {i}: replied: {}", query);
    }
}

#[tokio::main]
async fn main() {
    zenoh_util::try_init_log_from_env();

    let config_path = env::args().nth(1).unwrap();
    let config = Config::from_file(config_path).unwrap();
    let session = Arc::new(zenoh::open(config).res().await.unwrap());

    let mut handles = Vec::new();
    for i in 0..10 {
        handles.push(task::spawn(declare_queryable(session.clone(), i)));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
