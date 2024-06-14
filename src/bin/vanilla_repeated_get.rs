use std::env;

use tokio::time;
use zenoh::prelude::r#async::*;

const KEY_EXPR: &str = "slow_queryable";

#[tokio::main]
async fn main() {
    zenoh_util::try_init_log_from_env();

    let config_path = env::args().nth(1).unwrap();
    let config = Config::from_file(config_path).unwrap();
    let session = zenoh::open(config).res().await.unwrap();

    for i in 0..5 {
        let replies = session
            .get(Selector::try_from(KEY_EXPR).unwrap())
            .res()
            .await
            .unwrap();
        eprintln!("repeated_get: query: #{i}");
        while let Ok(reply) = replies.recv_async().await {
            match reply.sample {
                Ok(sample) => eprintln!("repeated_get: ok: {sample}"),
                Err(value) => eprintln!("repeated_get: err: {value}"),
            };
        }
        time::sleep(time::Duration::from_secs(10)).await;
    }

    loop {}
}
