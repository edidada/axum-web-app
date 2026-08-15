from fastapi import APIRouter, Depends, Request
from sqlalchemy.orm import Session

from models.base import get_db
from models.schemas import SetmealPageRequest
from .service import SetmealService

router = APIRouter()
service = SetmealService()


@router.get("/setmeal/select-page")
@router.post("/setmeal/select-page")
async def select_page(
    request: Request,
    req: SetmealPageRequest = None,
    db: Session = Depends(get_db)
):
    """套餐分页查询"""
    if req is None:
        req = SetmealPageRequest()
    return service.select_page(db, req)
