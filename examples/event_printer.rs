use tokio::stream::StreamExt;
use syncthing::{Client, Fallible};

static API_KEY: &str = include_str!("../api.key");

#[tokio::main]
async fn main() -> Fallible<()> {
    let client = Client::new(API_KEY);
    let mut stream = client.subscribe_to_all();

    while let Some(event) = stream.next().await {
        println!("{:?}", event?);
    }

    Ok(())
}
