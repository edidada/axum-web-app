use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthConfig {
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WebappConfig {
    pub port: u16,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: usize,
    pub issuer: String,
    pub audience: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SecurityConfig {
    pub cookie_secure: bool,
    pub allowed_origins: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SessionBackend {
    Database,
    Redis,
}

impl SessionBackend {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Database => "database",
            Self::Redis => "redis",
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SessionConfig {
    pub backend: SessionBackend,
    pub redis_url: String,
    pub redis_key_prefix: String,
}

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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub auth: AuthConfig,
    pub webapp: WebappConfig,
    pub jwt: JwtConfig,
    pub security: SecurityConfig,
    pub session: SessionConfig,
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
        settings.set_default("auth.role", "admin")?;
        settings.set_default("webapp.port", 9002)?;
        settings.set_default(
            "jwt.secret",
            "change-this-development-jwt-secret-before-production",
        )?;
        settings.set_default("jwt.expiration", 900)?;
        settings.set_default("jwt.issuer", "axum-web-app")?;
        settings.set_default("jwt.audience", "axum-web-frontend")?;
        settings.set_default("security.cookie_secure", false)?;
        settings.set_default("session.backend", "database")?;
        settings.set_default("session.redis_url", "redis://127.0.0.1:6379/0")?;
        settings.set_default("session.redis_key_prefix", "axum:auth_session:")?;
        settings.set_default("database.db_type", "sqlite")?;

        settings.merge(config::File::with_name("config/default").required(true))?;
        settings.merge(config::File::with_name(&format!("config/{profile}")).required(true))?;
        // 例如 APP_JWT__SECRET、APP_AUTH__PASSWORD、APP_SECURITY__COOKIE_SECURE。
        settings.merge(
            config::Environment::with_prefix("APP")
                .separator("__")
                .ignore_empty(true),
        )?;
        settings.try_into()
    }

    pub fn validate_security(&self) -> Result<(), String> {
        if self.jwt.secret.as_bytes().len() < 32 {
            return Err("jwt.secret must contain at least 32 bytes".to_string());
        }
        if self.jwt.expiration == 0 {
            return Err("jwt.expiration must be greater than zero".to_string());
        }
        if self.auth.role != "admin" && self.auth.role != "viewer" {
            return Err("auth.role must be admin or viewer".to_string());
        }
        if self.security.allowed_origins.is_empty()
            || self.security.allowed_origins.iter().any(|origin| {
                origin == "*" || !(origin.starts_with("http://") || origin.starts_with("https://"))
            })
        {
            return Err(
                "security.allowed_origins must contain explicit http(s) origins".to_string(),
            );
        }
        if self.session.backend == SessionBackend::Redis {
            if self.session.redis_url.trim().is_empty() {
                return Err("session.redis_url must not be empty for the Redis backend".to_string());
            }
            if self.session.redis_key_prefix.trim().is_empty() {
                return Err(
                    "session.redis_key_prefix must not be empty for the Redis backend".to_string(),
                );
            }
        }
        Ok(())
    }

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
        assert_eq!(config.database.db_type, "sqlite");
        assert_eq!(config.session.backend, SessionBackend::Database);
        assert_eq!(config.webapp.port, 9002);
        config.validate_security().expect("valid security defaults");
    }
}
