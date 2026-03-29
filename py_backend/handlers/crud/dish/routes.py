from fastapi import APIRouter, Depends, Request
from sqlalchemy.orm import Session

from models.base import get_db
from models.schemas import DishPageRequest, Response
from .service import DishService

router = APIRouter()
service = DishService()


@router.get("/dish/select-page")
@router.post("/dish/select-page")
async def select_page(
    request: Request,
    req: DishPageRequest = None,
    db: Session = Depends(get_db)
):
    """菜品分页查询"""
    if req is None:
        req = DishPageRequest()
    return service.select_page(db, req)


@router.post("/dish/insert")
async def insert(
    data: dict,
    db: Session = Depends(get_db)
):
    """菜品新增"""
    try:
        result = service.insert(db, data)
        return result
    except Exception as e:
        return Response(code=1, status="error", message=str(e))


@router.post("/dish/update")
async def update(
    data: dict,
    db: Session = Depends(get_db)
):
    """菜品更新"""
    try:
        result = service.update(db, data)
        return result
    except ValueError as e:
        return Response(code=1, status="error", message=str(e))
    except Exception as e:
        return Response(code=1, status="error", message=str(e))


@router.delete("/dish/delete/{id}")
async def delete(
    id: str,
    db: Session = Depends(get_db)
):
    """菜品删除"""
    try:
        service.delete(db, id)
        return Response(message="删除成功")
    except ValueError as e:
        return Response(code=1, status="error", message=str(e))
    except Exception as e:
        return Response(code=1, status="error", message=str(e))
