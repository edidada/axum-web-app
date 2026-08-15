use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AuthConfig {
    pub username: String,
    pub password: String,
}
#[derive(Clone, Deserialize)]
pub struct WebappConfig {
    pub port: u16,
    pub redisurl: String,
}
#[derive(Clone, Deserialize)]
pub struct RedisConfig {
    pub ip: String,
    pub port: u16,
    pub expiration: usize,
    pub key_prefix: String,
}

#[derive(Clone, Deserialize)]
pub struct DatabaseConfig {
    pub db_type: String,
    pub url: String,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub auth: AuthConfig,
    pub webapp: WebappConfig,
    pub redis: RedisConfig,
    pub database: DatabaseConfig,
}

impl Config {
    pub fn from_file() -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::default();
        
        // 设置默认值
        settings.set_default("auth.username", "user")?;
        settings.set_default("auth.password", "password")?;
        settings.set_default("webapp.port", 9001)?;
        settings.set_default("webapp.redisurl", "redis://127.0.0.1:6379")?;
        settings.set_default("redis.ip", "127.0.0.1")?;
        settings.set_default("redis.port", 6379)?;
        settings.set_default("redis.expiration", 3600)?;
        settings.set_default("redis.key_prefix", "session:")?;
        settings.set_default("database.db_type", "sqlite")?;
        settings.set_default("database.url", "sqlite://./test.db")?;
        
        // 配置文件缺失或格式非法均明确返回错误，防止静默使用错误的默认配置。
        settings.merge(config::File::with_name("config/default").required(true))?;
        
        settings.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::from_file().expect("load config/default.toml");
        assert_eq!(config.auth.username, "edidada");
        assert_eq!(config.webapp.port, 9002);
        assert_eq!(config.database.db_type, "sqlite");
    }
}
