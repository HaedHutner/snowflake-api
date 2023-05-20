use core::panic;

#[derive(Clone)]
pub struct Config {
    pub machine_id: u16,
    pub timezone: SnowflakeTimezone,
    pub port: u16,
    pub redis_connection_string: Option<String>
}

#[derive(Clone)]
pub enum SnowflakeTimezone {
    UTC,
    Local,
}

impl Config {
    pub fn from_env() -> Config {
        let config = Config {
            machine_id: std::env::var("MACHINE_ID").expect("Must provide machine id with the environment variable 'MACHINE_ID'").parse::<u16>().unwrap(),

            timezone: {
                match std::env::var("TIMEZONE") {
                    Ok(value) => {
                        match value.as_str() {
                            "UTC" =>   SnowflakeTimezone::UTC,
                            "Local" => SnowflakeTimezone::Local,
                            _ => {
                                println!("Invalid timezone '{value}' provided. Valid values are 'UTC' and 'Local'. Defaulting to UTC.");

                                SnowflakeTimezone::UTC
                            }
                        }
                    },
                    Err(_e) => {
                        println!("Timezone not configured. Defaulting to UTC.");

                        SnowflakeTimezone::UTC
                    }
                }
            },

            port: std::env::var("PORT").expect("Must provide application port with the environment variable 'PORT'").parse::<u16>().unwrap(),

            redis_connection_string: {
                match std::env::var("REDIS_CONNECTION_STRING") {
                    Ok(value) => Option::from(value),
                    Err(_e) => {
                        println!("Redis connection string not supplied. Defaulting to in-memory sequence tracking.");
                        
                        Option::None
                    }
                }
            }
        };

        if config.machine_id > 0b1111111111 {
            panic!("Provided MACHINE_ID must not be larger than 1023 ( 10-bit unsigned max ).");
        }

        config
    }
}