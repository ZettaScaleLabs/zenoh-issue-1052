use zenoh::prelude::r#async::*;

const KEY_EXPR: &str = "slow_queryable";

#[tokio::main]
async fn main() {
    zenoh_util::try_init_log_from_env();

    let mut config = Config::default();
    config.connect.endpoints = vec!["tcp/localhost:7447".parse::<EndPoint>().unwrap()];
    config.listen.endpoints = vec!["tcp/localhost:7446".parse::<EndPoint>().unwrap()];
    let session = zenoh::open(config).res().await.unwrap();

    let replies = session
        .get(Selector::try_from(KEY_EXPR).unwrap())
        .res()
        .await
        .unwrap();

    let Ok(reply) = replies.recv_async().await else {
        eprintln!("one_get: failed to receive reply");
        return;
    };

    match reply.sample {
        Ok(sample) => eprintln!("one_get: got sample {sample}"),
        Err(value) => eprintln!("one_get: got value {value}"),
    };
}
