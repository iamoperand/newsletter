#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // initialise our own configuration reader
    let mut settings = config::Config::default();
    // add configuration values from a file named "configuration".
    // it will look for any top-level file with an extension that "config" knows how to parse: yaml, json, etc.
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}
