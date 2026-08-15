package config

import (
	"fmt"
	"os"

	"gopkg.in/yaml.v3"
)

type AuthConfig struct {
	Username string `yaml:"username"`
	Password string `yaml:"password"`
}

type WebappConfig struct {
	Port     int    `yaml:"port"`
	RedisURL string `yaml:"redisurl"`
}

type RedisConfig struct {
	IP         string `yaml:"ip"`
	Port       int    `yaml:"port"`
	Expiration int    `yaml:"expiration"`
	KeyPrefix  string `yaml:"key_prefix"`
}

type DatabaseConfig struct {
	DBType   string `yaml:"db_type"`
	Host     string `yaml:"host"`
	Port     int    `yaml:"port"`
	Username string `yaml:"username"`
	Password string `yaml:"password"`
	Database string `yaml:"database"`
}

type MinioConfig struct {
	Endpoint  string `yaml:"endpoint"`
	AccessKey string `yaml:"access_key"`
	SecretKey string `yaml:"secret_key"`
	UseSSL    bool   `yaml:"use_ssl"`
	Bucket    string `yaml:"bucket"`
}

type Config struct {
	Auth     AuthConfig     `yaml:"auth"`
	Webapp   WebappConfig   `yaml:"webapp"`
	Redis    RedisConfig    `yaml:"redis"`
	Database DatabaseConfig `yaml:"database"`
	Minio    MinioConfig    `yaml:"minio"`
}

var globalConfig *Config

func LoadConfig(path string) (*Config, error) {
	if globalConfig != nil {
		return globalConfig, nil
	}

	data, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read config file: %w", err)
	}

	var cfg Config
	if err := yaml.Unmarshal(data, &cfg); err != nil {
		return nil, fmt.Errorf("failed to parse config file: %w", err)
	}

	globalConfig = &cfg
	return globalConfig, nil
}

func GetConfig() *Config {
	if globalConfig == nil {
		// 尝试加载默认配置
		cfg, err := LoadConfig("config/config.yaml")
		if err != nil {
			// 返回默认配置
			return &Config{
				Auth: AuthConfig{
					Username: "edidada",
					Password: "Pass123Qwe",
				},
				Webapp: WebappConfig{
					Port:     9003,
					RedisURL: "redis://127.0.0.1:6379",
				},
				Redis: RedisConfig{
					IP:         "127.0.0.1",
					Port:       6379,
					Expiration: 3600,
					KeyPrefix:  "session:",
				},
				Database: DatabaseConfig{
					DBType:   "postgresql",
					Host:     "127.0.0.1",
					Port:     5432,
					Username: "ibqo",
					Password: "",
					Database: "axum_db",
				},
				Minio: MinioConfig{
					Endpoint:  "localhost:9000",
					AccessKey: "minioadmin",
					SecretKey: "minioadmin",
					UseSSL:    false,
					Bucket:    "images",
				},
			}
		}
		return cfg
	}
	return globalConfig
}

func (c *Config) GetDatabaseDSN() string {
	switch c.Database.DBType {
	case "postgresql", "postgres", "pg":
		// 使用 URL 格式
		password := c.Database.Password
		if password != "" {
			password = ":" + password
		}
		return fmt.Sprintf("postgres://%s%s@%s:%d/%s?sslmode=disable",
			c.Database.Username, password, c.Database.Host, c.Database.Port, c.Database.Database)
	case "mysql":
		return fmt.Sprintf("%s:%s@tcp(%s:%d)/%s?charset=utf8mb4&parseTime=True&loc=Local",
			c.Database.Username, c.Database.Password, c.Database.Host, c.Database.Port, c.Database.Database)
	default:
		password := c.Database.Password
		if password != "" {
			password = ":" + password
		}
		return fmt.Sprintf("postgres://%s%s@%s:%d/%s?sslmode=disable",
			c.Database.Username, password, c.Database.Host, c.Database.Port, c.Database.Database)
	}
}

func (c *Config) IsPostgreSQL() bool {
	return c.Database.DBType == "postgresql" || c.Database.DBType == "postgres" || c.Database.DBType == "pg"
}

func (c *Config) IsMySQL() bool {
	return c.Database.DBType == "mysql"
}
