use std::{time::{SystemTime, UNIX_EPOCH}, fmt::Display, error::Error};

use redis::{self, AsyncCommands};

use crate::config::Config as SnowflakeConfig;

pub(crate) struct SnowflakeGenerator {
    config: SnowflakeConfig,
    redis_client: RedisClient
}

impl SnowflakeGenerator{
    pub(crate) fn new(config: SnowflakeConfig, redis_client: RedisClient) -> Self {
        SnowflakeGenerator {
            config,
            redis_client
        }
    }

    pub async fn generate_new(&self) -> Result<i64, SnowflakeError> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards??").as_millis();

        match self.fetch_sequence(self.config.machine_id, time).await {
            Ok(result) => Ok(((time << (64 - 42)) as i64) | (self.config.machine_id as i64) << (64 - (42 + 10)) | result),
            Err(e) => Err(SnowflakeError::new("Couldn't generate snowflake id", Box::from(e)))
        }
    }

    async fn fetch_sequence(&self, machine_id: u16, unix_millis: u128) -> Result<i64, SnowflakeError> {
        match self.redis_client.get_sequence(machine_id, unix_millis).await {
            Ok(result) => Ok(result as i64),
            Err(e) => Err(e)
        }
    }
}

#[derive(Debug)]
pub(crate) struct SnowflakeError {
    message: String,
    internal: Box<dyn std::error::Error>
}

impl SnowflakeError {
    pub(crate) fn new(message: &str, error: Box<dyn Error>) -> Self {
        SnowflakeError { message: String::from(message), internal: Box::from(error) }
    }
}

impl Error for SnowflakeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.internal.source()
    }

    fn description(&self) -> &str {
        "deprecated"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for SnowflakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ( {} ): {}", std::file!(), self.message, self.internal)
    }
}

pub(crate) struct RedisClient {
    redis: redis::Client
}

impl RedisClient {
    pub(crate) fn new(config: &SnowflakeConfig) -> Self {
        RedisClient {
            redis: redis::Client::open(config.redis_connection_string.as_str()).unwrap()
        }
    }

    async fn get_sequence(&self, machine_id: u16, unix_millis: u128) -> Result<u16, SnowflakeError> {
        match self.redis.get_async_connection().await {
            Ok(mut conn) => {
                let key = format!("{unix_millis}_{machine_id}");

                match conn.get::<&String, u16>(&key).await {
                    Ok(result) => Ok(result),
                    Err(_e) => {
                        let _resp = match conn.set_ex::<&String, u16, u16>(&key, 0u16, 1usize).await {
                            Ok(res) => Ok(res),
                            Err(e) => Err(SnowflakeError::new("Couldn't fetch sequence", Box::from(e)))
                        };

                        Ok(0u16)
                    }
                }
            },
            Err(e) => {
                Err(SnowflakeError::new("Couldn't create redis connection", Box::from(e)))
            }
        }
    }
}