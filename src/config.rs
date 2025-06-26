use serde::Deserialize;
use std::net::{Ipv6Addr, SocketAddr};

#[derive(Deserialize)]
pub struct Configuration {
    pub env: Environment,
    pub listen_address: SocketAddr,
    pub app_port: u16,
}

impl Configuration {
    pub fn new() -> Self {
        let env = env_var("APP_ENVIRONMENT")
            .try_into()
            .expect("APP_ENVIRONMENT is invalid or not specified.");
        let app_port = env_var("PORT").parse::<u16>().expect(
            "PORT is invalid or not specified. Please specify a valid unsigned 16-bit integer",
        );
        let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, app_port));

        Configuration {
            env,
            listen_address,
            app_port,
        }
    }
}

// Current execution Environment Context.
#[derive(Deserialize)]
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
