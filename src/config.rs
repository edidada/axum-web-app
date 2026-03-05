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
pub struct RedisConfig {
    pub ip: String,
    pub port: u16,
    pub expiration: usize,
    pub key_prefix: String,
}
#[derive(Deserialize)]
pub struct Config {
    pub auth: AuthConfig,
    pub webapp: WebappConfig,
    pub redis: RedisConfig,
}

impl Config {
    pub fn from_file() -> Self {
        let mut settings = config::Config::default();
        
        // 设置默认值
        settings.set("auth.username", "user").unwrap();
        settings.set("auth.password", "password").unwrap();
        settings.set("webapp.port", 9001).unwrap();
        settings.set("webapp.redisurl", "redis://127.0.0.1:6379").unwrap();
        settings.set("redis.ip", "127.0.0.1").unwrap();
        settings.set("redis.port", 6379).unwrap();
        settings.set("redis.expiration", 3600).unwrap();
        settings.set("redis.key_prefix", "session:").unwrap();
        
        // 尝试加载配置文件，如果不存在则使用默认值
        let _ = settings.merge(config::File::with_name("config/default"));
        
        settings.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::from_file();
        assert_eq!(config.auth.username, "user");
        assert_eq!(config.auth.password, "password");
    }
}