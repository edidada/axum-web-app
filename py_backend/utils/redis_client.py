import redis
from config.settings import settings

# Redis 客户端
redis_client = redis.Redis(
    host=settings.redis_host,
    port=settings.redis_port,
    db=settings.redis_db,
    password=settings.redis_password,
    decode_responses=True
)


def get_redis() -> redis.Redis:
    """获取 Redis 客户端"""
    return redis_client


def set_session(session_id: str, user_id: str, expire: int = 3600):
    """设置会话"""
    redis_client.setex(f"session:{session_id}", expire, user_id)


def get_session(session_id: str) -> str:
    """获取会话"""
    return redis_client.get(f"session:{session_id}")


def delete_session(session_id: str):
    """删除会话"""
    redis_client.delete(f"session:{session_id}")
