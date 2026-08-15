from fastapi import APIRouter
from models.schemas import Response
from utils.redis_client import get_redis
from sqlalchemy import text
from models.base import engine

router = APIRouter()


@router.get("/health")
async def health_check():
    """健康检查"""
    return Response(data="OK")


@router.get("/health/detailed")
async def health_check_detailed():
    """详细健康检查"""
    return Response(data={
        "status": "healthy",
        "components": {
            "api": "up",
            "database": "up",
            "redis": "up"
        }
    })


@router.get("/ready")
async def readiness_check():
    """就绪检查"""
    try:
        # 检查数据库连接
        with engine.connect() as conn:
            conn.execute(text("SELECT 1"))
        
        # 检查 Redis
        get_redis().ping()
        
        return Response(data="ready")
    except Exception as e:
        return Response(code=1, status="error", message=str(e))


@router.get("/live")
async def liveness_check():
    """存活检查"""
    return Response(data="alive")
