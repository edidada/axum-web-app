from fastapi import APIRouter, Depends, Request
from sqlalchemy.orm import Session

from models.base import get_db
from models.schemas import SysDictPageRequest, Response
from .service import SysDictService

router = APIRouter()
service = SysDictService()


@router.get("/sys-dict/select-page")
@router.post("/sys-dict/select-page")
async def select_page(
    request: Request,
    req: SysDictPageRequest = None,
    db: Session = Depends(get_db)
):
    """字典分页查询"""
    if req is None:
        req = SysDictPageRequest()
    return service.select_page(db, req)


@router.post("/sys-dict/insert")
async def insert(
    data: dict,
    db: Session = Depends(get_db)
):
    """字典新增"""
    try:
        result = service.insert(db, data)
        return result
    except Exception as e:
        return Response(code=1, status="error", message=str(e))


@router.post("/sys-dict/update")
async def update(
    data: dict,
    db: Session = Depends(get_db)
):
    """字典更新"""
    try:
        result = service.update(db, data)
        return result
    except ValueError as e:
        return Response(code=1, status="error", message=str(e))
    except Exception as e:
        return Response(code=1, status="error", message=str(e))


@router.delete("/sys-dict/delete/{id}")
async def delete(
    id: str,
    db: Session = Depends(get_db)
):
    """字典删除"""
    try:
        service.delete(db, id)
        return Response(message="删除成功")
    except ValueError as e:
        return Response(code=1, status="error", message=str(e))
    except Exception as e:
        return Response(code=1, status="error", message=str(e))
