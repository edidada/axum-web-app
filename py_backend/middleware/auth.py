from fastapi import Request, HTTPException, status
from fastapi.responses import JSONResponse
from starlette.middleware.base import BaseHTTPMiddleware
from utils.redis_client import get_session
from config.settings import settings
import time


class AuthMiddleware(BaseHTTPMiddleware):
    """认证中间件"""
    
    async def dispatch(self, request: Request, call_next):
        # 跳过认证的路径
        public_paths = [
            "/health",
            "/health/detailed",
            "/ready",
            "/live",
            "/api/login",
            "/docs",
            "/openapi.json",
            "/redoc"
        ]
        
        path = request.url.path
        
        # 检查是否是公开路径
        if any(path.startswith(p) for p in public_paths):
            return await call_next(request)
        
        # 获取 session ID
        session_id = request.cookies.get("session")
        if not session_id:
            # 尝试从 header 获取
            auth_header = request.headers.get("Authorization")
            if auth_header and auth_header.startswith("Bearer "):
                session_id = auth_header[7:]
        
        if not session_id:
            return JSONResponse(
                status_code=401,
                content={"code": 1, "status": "error", "message": "未登录"}
            )
        
        # 验证 session
        user_id = get_session(session_id)
        if not user_id:
            return JSONResponse(
                status_code=401,
                content={"code": 1, "status": "error", "message": "会话已过期"}
            )
        
        # 将用户信息存入请求状态
        request.state.user_id = user_id
        request.state.session_id = session_id
        
        return await call_next(request)


class CORSMiddleware(BaseHTTPMiddleware):
    """CORS 中间件"""
    
    async def dispatch(self, request: Request, call_next):
        if request.method == "OPTIONS":
            return JSONResponse(
                content={},
                headers={
                    "Access-Control-Allow-Origin": "*",
                    "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS",
                    "Access-Control-Allow-Headers": "Content-Type, Authorization",
                }
            )
        
        response = await call_next(request)
        response.headers["Access-Control-Allow-Origin"] = "*"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, PUT, DELETE, OPTIONS"
        response.headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
        
        return response
