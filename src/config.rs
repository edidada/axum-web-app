//! Application configuration.
//!
//! Loads `config/default.toml` together with the profile selected by
//! `APP_PROFILE` (defaults to `sqlite`) and builds the database URL.

use serde::{Deserialize, Serialize};

/// Credentials accepted by the login endpoint.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthConfig {
    pub username: String,
    pub password: String,
}

/// Web server settings.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WebappConfig {
    pub port: u16,
    pub redisurl: String,
}

/// Redis connection settings used for session storage.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RedisConfig {
    pub ip: String,
    pub port: u16,
    pub expiration: usize,
    pub key_prefix: String,
}

/// HS256 signing settings for JSON Web Tokens.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: usize,
}

/// Database connection settings for the active profile.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DbConfig {
    pub db_type: String,
    pub url: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
}

/// Fully resolved application settings shared through an axum `Extension`.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub auth: AuthConfig,
    pub webapp: WebappConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub database: DbConfig,
}

impl Config {
    /// 加载公共配置与由 APP_PROFILE 指定的数据库 profile。
    /// 可选 profile：postgresql、mysql、sqlite（默认）。
    pub fn from_file() -> Result<Self, config::ConfigError> {
        let profile = std::env::var("APP_PROFILE").unwrap_or_else(|_| "sqlite".to_string());
        let mut settings = config::Config::default();

        settings.set_default("auth.username", "edidada")?;
        settings.set_default("auth.password", "Pass123Qwe")?;
        settings.set_default("webapp.port", 9002)?;
        settings.set_default("webapp.redisurl", "redis://127.0.0.1:6379")?;
        settings.set_default("redis.ip", "127.0.0.1")?;
        settings.set_default("redis.port", 6379)?;
        settings.set_default("redis.expiration", 3600)?;
        settings.set_default("redis.key_prefix", "session:")?;
        settings.set_default(
            "jwt.secret",
            "change-this-development-jwt-secret-before-production",
        )?;
        settings.set_default("jwt.expiration", 3600)?;
        settings.set_default("database.db_type", "sqlite")?;

        settings.merge(config::File::with_name("config/default").required(true))?;
        settings.merge(config::File::with_name(&format!("config/{profile}")).required(true))?;
        settings.try_into()
    }

    /// Builds the connection URL for the configured database driver.
    pub fn get_database_url(&self) -> Result<String, String> {
        if self.database.db_type == "sqlite" {
            return self
                .database
                .url
                .clone()
                .ok_or_else(|| "sqlite profile requires database.url".to_string());
        }

        let host = self
            .database
            .host
            .as_deref()
            .ok_or("database.host is required")?;
        let port = self.database.port.ok_or("database.port is required")?;
        let username = self
            .database
            .username
            .as_deref()
            .ok_or("database.username is required")?;
        let password = self
            .database
            .password
            .as_deref()
            .ok_or("database.password is required")?;
        let database = self
            .database
            .database
            .as_deref()
            .ok_or("database.database is required")?;
        match self.database.db_type.as_str() {
            "postgresql" | "postgres" | "pg" => Ok(format!(
                "postgres://{username}:{password}@{host}:{port}/{database}"
            )),
            "mysql" => Ok(format!(
                "mysql://{username}:{password}@{host}:{port}/{database}"
            )),
            other => Err(format!("unsupported database type: {other}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_profile_is_sqlite() {
        std::env::remove_var("APP_PROFILE");
        let config = Config::from_file().expect("load sqlite profile");
        assert_eq!(
            config.database.db_type, "sqlite",
            "the default profile should be sqlite"
        );
        assert_eq!(config.webapp.port, 9002, "the default port should be 9002");
    }
}
