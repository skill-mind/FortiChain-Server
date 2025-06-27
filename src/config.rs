use serde::Deserialize;
use std::{
    net::{Ipv6Addr, SocketAddr},
    sync::Arc,
};

// Type alias for thread safe app configuration.
pub type Config = Arc<Configuration>;
#[derive(Deserialize, Clone)]
pub struct Configuration {
    pub env: Environment,
    pub listen_address: SocketAddr,
    pub app_port: u16,
    pub database_url: String,
    pub max_db_connections: u32,
}

impl Configuration {
    pub fn new() -> Config {
        let env = env_var("APP_ENVIRONMENT")
            .try_into()
            .expect("APP_ENVIRONMENT is invalid or not specified.");
        let app_port = env_var("PORT").parse::<u16>().expect(
            "PORT is invalid or not specified. Please specify a valid unsigned 16-bit integer",
        );

        let database_url = env_var("DATABASE_URL");
        let max_db_connections = env_var("DB_MAX_CONNECTIONS")
            .parse()
            .expect("DB_MAX_CONNECTIONS is invalid or not specified.");
        let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, app_port));

        // Configuration values to be safely shared across requests.
        Arc::new(Configuration {
            env,
            listen_address,
            app_port,
            database_url,
            max_db_connections,
        })
    }
}

// Current execution Environment Context.
#[derive(Deserialize, Clone)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            weird => Err(format!(
                "{} is not a supported environment. \
                Use either `local` or `production`.",
                weird
            )),
        }
    }
}

pub fn env_var(name: &str) -> String {
    std::env::var(name)
        .map_err(|e| format!("{}: {}", name, e))
        .expect("Missing environment variable")
}
