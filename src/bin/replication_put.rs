use tokio::time;
use zenoh::prelude::r#async::*;

const KEY_EXPR: &str = "zenoh/issues/1052";
const VALUE: &str = "📁";

#[tokio::main]
async fn main() {
    zenoh_util::try_init_log_from_env();

    let mut config = Config::default();
    config.connect.endpoints = vec!["tcp/localhost:7447".parse::<EndPoint>().unwrap()];
    let session = zenoh::open(config).res().await.unwrap();

    for i in 0..6000 {
        let _ = session
            .put(
                KeyExpr::try_from(&format!("{KEY_EXPR}/{i}")).unwrap(),
                VALUE,
            )
            .congestion_control(CongestionControl::Block)
            .res()
            .await
            .unwrap();

        eprintln!("put: #{i}");
        time::sleep(time::Duration::from_millis(10)).await;
    }
}
