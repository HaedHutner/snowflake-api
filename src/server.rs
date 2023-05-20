use std::net::SocketAddr;

use axum::{
    routing::get,
    Router,
};

use crate::config::Config as SnowflakeConfig;
use crate::snowflake::{SnowflakeGenerator, SequenceTracker};

pub async fn start_server() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(get_snowflake_id));

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_snowflake_id() -> String {
    let config = SnowflakeConfig::from_env();
    let redis_client = SequenceTracker::new(&config);
    let snowflake_generator = SnowflakeGenerator::new();

    match snowflake_generator.generate_new(&config, &redis_client).await {
        Ok(res) => format!("{}", res),
        Err(e) => format!("{}", e)
    }
}