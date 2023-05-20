use std::cell::RefCell;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};

use crate::config::Config as SnowflakeConfig;
use crate::snowflake::{SnowflakeGenerator, RedisClient};

pub(crate) struct Services{
    config: SnowflakeConfig,
    snowflake_generator: SnowflakeGenerator,
    redis_client: RedisClient
}

impl Services {
    pub fn new() -> Self {
        let config = SnowflakeConfig::from_env();
        let redis_client = RedisClient::new(&config);
        let snowflake_generator = SnowflakeGenerator::new(&config, &redis_client);

        Services {
            config: &config, 
            redis_client: &redis_client, 
            snowflake_generator: &snowflake_generator
        }
    }
}

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
    match SERVICES.snowflake_generator.generate_new().await {
        Ok(res) => format!("{}", res),
        Err(e) => format!("{}", e)
    }
}