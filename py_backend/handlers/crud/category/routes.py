from fastapi import APIRouter, Depends, Request
from sqlalchemy.orm import Session

from models.base import get_db
from models.schemas import CategoryPageRequest, Response
from .service import CategoryService

router = APIRouter()
service = CategoryService()


@router.get("/category/select-page")
@router.post("/category/select-page")
async def select_page(
    request: Request,
    req: CategoryPageRequest = None,
    db: Session = Depends(get_db)
):
    """分类分页查询"""
    if req is None:
        req = CategoryPageRequest()
    return service.select_page(db, req)


@router.post("/category/insert")
async def insert(
    data: dict,
    db: Session = Depends(get_db)
):
    """分类新增"""
    try:
        result = service.insert(db, data)
        return result
    except Exception as e:
        return Response(code=1, status="error", message=str(e))


@router.post("/category/update")
async def update(
    data: dict,
    db: Session = Depends(get_db)
):
    """分类更新"""
    try:
        result = service.update(db, data)
        return result
    except ValueError as e:
        return Response(code=1, status="error", message=str(e))
    except Exception as e:
        return Response(code=1, status="error", message=str(e))


@router.delete("/category/delete/{id}")
async def delete(
    id: str,
    db: Session = Depends(get_db)
):
    """分类删除"""
    try:
        service.delete(db, id)
        return Response(message="删除成功")
    except ValueError as e:
        return Response(code=1, status="error", message=str(e))
    except Exception as e:
        return Response(code=1, status="error", message=str(e))
