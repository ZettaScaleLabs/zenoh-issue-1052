use futures::FutureExt;
use tokio::io::AsyncReadExt;
use zenoh::{prelude::*, Config};

#[tokio::main]
async fn main() {
    zenoh_util::try_init_log_from_env();

    let session = zenoh::open(Config::default()).await.unwrap();

    let queryable = session.declare_queryable("test").await.unwrap();

    let mut input = Vec::new();
    let mut stdin = tokio::io::stdin();

    loop {
        tokio::select!(
            _query = queryable.recv_async() => {},
            _ = stdin.read_exact(&mut input).fuse() => {
                let replies = session.get("test").await.unwrap();
                while let Ok(_reply) = replies.recv_async().await {

                }
                return
            },
        );
    }
}
