from fastapi import APIRouter, Request, Response as FastAPIResponse
from models.schemas import LoginRequest, LoginResponse, Response
from models.models import Employee
from models.base import get_db
from utils.redis_client import set_session, delete_session
from utils.id_generator import generate_id
from passlib.context import CryptContext
import secrets

router = APIRouter()
pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")


@router.post("/api/login")
async def login(request: LoginRequest, response: FastAPIResponse):
    """登录"""
    db = next(get_db())
    
    # 查找用户
    employee = db.query(Employee).filter(
        Employee.username == request.username,
        Employee.is_deleted == 0
    ).first()
    
    if not employee:
        return LoginResponse(code=1, status="error", message="用户名或密码错误")
    
    # 验证密码（明文比较，与Go版本一致）
    if employee.password != request.password:
        return LoginResponse(code=1, status="error", message="用户名或密码错误")
    
    # 生成 session ID
    session_id = secrets.token_urlsafe(32)
    
    # 保存到 Redis
    set_session(session_id, employee.id)
    
    # 设置 cookie
    response.set_cookie(
        key="session",
        value=session_id,
        httponly=True,
        max_age=3600,
        samesite="lax"
    )
    
    return LoginResponse(data={
        "id": employee.id,
        "name": employee.name,
        "username": employee.username,
        "session": session_id
    })


@router.post("/api/logout")
async def logout(request: Request, response: FastAPIResponse):
    """登出"""
    session_id = request.cookies.get("session")
    if session_id:
        delete_session(session_id)
    
    response.delete_cookie("session")
    return Response(message="登出成功")
