use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthConfig {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct WebappConfig {
    pub port: u16,
    pub redisurl: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub auth: AuthConfig,
    pub webapp: WebappConfig,
}

impl Config {
    pub fn from_file() -> Self {
        let mut settings = config::Config::default();
        settings.merge(config::File::with_name("config/default")).unwrap();
        settings.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::from_file();
        // assert_eq!(config.auth.username, "user");
        // assert_eq!(config.auth.password, "password");
    }
}