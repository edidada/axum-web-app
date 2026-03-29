from fastapi import APIRouter, Depends, Request
from sqlalchemy.orm import Session

from models.base import get_db
from models.schemas import UserPageRequest
from .service import UserService

router = APIRouter()
service = UserService()


@router.get("/user/select-page")
@router.post("/user/select-page")
async def select_page(
    request: Request,
    req: UserPageRequest = None,
    db: Session = Depends(get_db)
):
    """用户分页查询"""
    if req is None:
        req = UserPageRequest()
    return service.select_page(db, req)
