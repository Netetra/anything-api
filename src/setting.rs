use config::{Config, ConfigError, File};
use serde::Deserialize;

pub fn read_settings(file_name: &str) -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name(file_name))
        .build()?
        .try_deserialize()?;

    Ok(settings)
}

#[derive(Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DbSettings,
}

#[derive(Deserialize)]
pub struct ServerSettings {
    pub ip: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DbSettings {
    pub url: String,
}
