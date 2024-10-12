use syncthing::{Client, Fallible};

static API_KEY: &str = include_str!("../api.key");

#[tokio::main]
async fn main() -> Fallible<()> {
    let client = Client::new(API_KEY);
    let system = client.get_system_version().await?;

    println!("syncthing {} is running on {}!", system.version, system.os);

    Ok(())
}
