from fastapi import APIRouter, Depends, Request
from sqlalchemy.orm import Session

from models.base import get_db
from models.schemas import OrdersPageRequest
from .service import OrdersService

router = APIRouter()
service = OrdersService()


@router.get("/orders/select-page")
@router.post("/orders/select-page")
async def select_page(
    request: Request,
    req: OrdersPageRequest = None,
    db: Session = Depends(get_db)
):
    """订单分页查询"""
    if req is None:
        req = OrdersPageRequest()
    return service.select_page(db, req)
