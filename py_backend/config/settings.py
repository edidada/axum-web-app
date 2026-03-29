from pydantic_settings import BaseSettings
from typing import Optional


class Settings(BaseSettings):
    # 应用配置
    app_name: str = "Python Backend API"
    debug: bool = True
    host: str = "127.0.0.1"
    port: int = 9004
    
    # 数据库配置
    db_type: str = "postgresql"
    db_host: str = "127.0.0.1"
    db_port: int = 5432
    db_username: str = "ibqo"
    db_password: str = ""
    db_name: str = "axum_db"
    
    # Redis配置
    redis_host: str = "127.0.0.1"
    redis_port: int = 6379
    redis_db: int = 0
    redis_password: Optional[str] = None
    
    # MinIO配置
    minio_endpoint: str = "localhost:9000"
    minio_access_key: str = "minioadmin"
    minio_secret_key: str = "minioadmin"
    minio_bucket: str = "images"
    minio_use_ssl: bool = False
    
    # Session配置
    session_timeout: int = 3600  # 1小时
    
    class Config:
        env_file = ".env"
        env_file_encoding = "utf-8"
    
    @property
    def database_url(self) -> str:
        if self.db_type == "postgresql":
            return f"postgresql://{self.db_username}:{self.db_password}@{self.db_host}:{self.db_port}/{self.db_name}"
        else:
            return f"mysql+pymysql://{self.db_username}:{self.db_password}@{self.db_host}:{self.db_port}/{self.db_name}"


# 全局配置实例
settings = Settings()
