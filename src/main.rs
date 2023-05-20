mod snowflake;
mod config;
mod server;

use tokio;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    match server::start_server().await {
        Ok(_) => (),
        Err(_) => ()
    }
}