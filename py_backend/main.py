#!/usr/bin/env python3
"""
Python Backend API
功能与 axum/Go 后端相同
"""

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
import uvicorn

from config.settings import settings
from middleware.auth import AuthMiddleware, CORSMiddleware as CustomCORSMiddleware
from handlers import health, auth, image
from handlers.crud import (
    sys_dict, address_book, category, dish,
    employee, orders, setmeal, user
)

# 创建应用
app = FastAPI(
    title=settings.app_name,
    description="Python Backend API - 功能与 axum/Go 后端相同",
    version="1.0.0",
    docs_url="/docs",
    redoc_url="/redoc"
)

# 添加中间件
app.add_middleware(CustomCORSMiddleware)
app.add_middleware(AuthMiddleware)

# 注册路由
app.include_router(health.router)
app.include_router(auth.router)
app.include_router(image.router)

# 注册 CRUD 路由
app.include_router(sys_dict.router)
app.include_router(address_book.router)
app.include_router(category.router)
app.include_router(dish.router)
app.include_router(employee.router)
app.include_router(orders.router)
app.include_router(setmeal.router)
app.include_router(user.router)


@app.on_event("startup")
async def startup_event():
    """启动事件"""
    print("=" * 50)
    print("🚀 Python Backend Server Starting...")
    print("=" * 50)
    print(f"📡 Server: http://{settings.host}:{settings.port}")
    print(f"📚 API Docs: http://{settings.host}:{settings.port}/docs")
    print("=" * 50)


@app.on_event("shutdown")
async def shutdown_event():
    """关闭事件"""
    print("👋 Server shutting down...")


if __name__ == "__main__":
    uvicorn.run(
        "main:app",
        host=settings.host,
        port=settings.port,
        reload=settings.debug,
        log_level="info"
    )
