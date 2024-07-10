use zenoh::{config::EndPoint, query::Selector, Config};

const KEY_EXPR: &str = "slow_queryable";

#[tokio::main]
async fn main() {
    zenoh_util::try_init_log_from_env();

    let mut config = Config::default();
    config
        .connect
        .endpoints
        .set(vec!["tcp/localhost:7447".parse::<EndPoint>().unwrap()])
        .unwrap();
    config
        .listen
        .endpoints
        .set(vec!["tcp/localhost:7446".parse::<EndPoint>().unwrap()])
        .unwrap();
    let session = zenoh::open(config).await.unwrap();

    let replies = session
        .get(Selector::try_from(KEY_EXPR).unwrap())
        .await
        .unwrap();

    let Ok(reply) = replies.recv_async().await else {
        eprintln!("one_get: failed to receive reply");
        return;
    };

    match reply.into_result() {
        Ok(sample) => eprintln!("one_get: got sample {sample:?}"),
        Err(err) => eprintln!("one_get: got value {err:?}"),
    };
}
