use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthConfig {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WebappConfig {
    pub port: u16,
    pub redisurl: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RedisConfig {
    pub ip: String,
    pub port: u16,
    pub expiration: usize,
    pub key_prefix: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DbConfig {
    // 数据库类型: "mysql" 或 "postgresql"
    pub db_type: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub auth: AuthConfig,
    pub webapp: WebappConfig,
    pub redis: RedisConfig,
    // 保留 mysql 配置项以兼容旧配置
    pub mysql: DatabaseConfig,
    // 新增数据库配置项
    pub database: DbConfig,
}

impl Config {
    pub fn from_file() -> Self {
        let mut settings = config::Config::default();
        
        // 设置默认值
        settings.set("auth.username", "edidada").unwrap();
        settings.set("auth.password", "Pass123Qwe").unwrap();
        settings.set("webapp.port", 9002).unwrap();
        settings.set("webapp.redisurl", "redis://127.0.0.1:6379").unwrap();
        settings.set("redis.ip", "127.0.0.1").unwrap();
        settings.set("redis.port", 6379).unwrap();
        settings.set("redis.expiration", 3600).unwrap();
        settings.set("redis.key_prefix", "session:").unwrap();
        
        // 兼容旧配置 (mysql)
        settings.set("mysql.host", "127.0.0.1").unwrap();
        settings.set("mysql.port", 3306).unwrap();
        settings.set("mysql.username", "root").unwrap();
        settings.set("mysql.password", "").unwrap();
        settings.set("mysql.database", "ruiji").unwrap();
        
        // 新数据库配置 (支持 mysql 和 postgresql)
        settings.set("database.db_type", "mysql").unwrap();
        settings.set("database.host", "127.0.0.1").unwrap();
        settings.set("database.port", 3306).unwrap();
        settings.set("database.username", "root").unwrap();
        settings.set("database.password", "").unwrap();
        settings.set("database.database", "ruiji").unwrap();
        
        // 尝试加载配置文件，如果不存在则使用默认值
        let _ = settings.merge(config::File::with_name("config/default"));
        
        settings.try_into().unwrap()
    }
    
    /// 获取数据库连接字符串
    pub fn get_database_url(&self) -> String {
        match self.database.db_type.as_str() {
            "postgresql" | "postgres" | "pg" => {
                format!("postgres://{}:{}@{}:{}/{}",
                    self.database.username,
                    self.database.password,
                    self.database.host,
                    self.database.port,
                    self.database.database
                )
            }
            "mysql" | _ => {
                format!("mysql://{}:{}@{}:{}/{}",
                    self.database.username,
                    self.database.password,
                    self.database.host,
                    self.database.port,
                    self.database.database
                )
            }
        }
    }
    
    /// 检查是否为 PostgreSQL
    pub fn is_postgresql(&self) -> bool {
        matches!(self.database.db_type.as_str(), "postgresql" | "postgres" | "pg")
    }
    
    /// 检查是否为 MySQL
    pub fn is_mysql(&self) -> bool {
        matches!(self.database.db_type.as_str(), "mysql" | "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::from_file();
        assert!(!config.database.db_type.is_empty());
    }
}
