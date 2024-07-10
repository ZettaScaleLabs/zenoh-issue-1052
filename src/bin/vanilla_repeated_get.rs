use std::{env, time::Duration};

use zenoh::{query::Selector, Config};

const KEY_EXPR: &str = "slow_queryable";

#[tokio::main]
async fn main() {
    let config_path = env::args().nth(1).unwrap();
    let interval = env::args().nth(2).unwrap().parse::<u64>().unwrap();

    let config = Config::from_file(config_path).unwrap();
    let session = zenoh::open(config).await.unwrap();

    loop {
        let replies = session
            .get(Selector::try_from(KEY_EXPR).unwrap())
            .await
            .unwrap();
        while let Ok(_) = replies.recv_async().await {}
        tokio::time::sleep(Duration::from_millis(interval)).await;
    }
}
