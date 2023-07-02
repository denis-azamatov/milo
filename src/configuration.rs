use std::env;

use secrecy::{ExposeSecret, Secret};

/// Aplication level settings
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub name: String,
}

/// Custom environment variable to detect local tests run.
const MILO_ENV: &str = "MILO_ENV";

#[derive(PartialEq, Eq)]
enum MiloEnv {
    Test,
    None,
}

impl MiloEnv {
    fn get() -> MiloEnv {
        if let Ok(var) = env::var(MILO_ENV) {
            match var.as_str() {
                "Test" => MiloEnv::Test,
                _ => MiloEnv::None,
            }
        } else {
            MiloEnv::None
        }
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut builder = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .add_source(config::File::with_name("configuration.local").required(false));

    if MiloEnv::get() == MiloEnv::Test {
        builder = builder.add_source(config::File::with_name("configuration.test"));
    }

    let settings = builder.build()?;

    settings.try_deserialize()
}
