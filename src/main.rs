mod snowflake;
mod config;
mod server;

use tokio;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let services = server::Services::new();

    match server::start_server(services).await {
        Ok(_) => (),
        Err(_) => ()
    }
}