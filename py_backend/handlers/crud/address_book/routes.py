from fastapi import APIRouter, Depends, Request
from sqlalchemy.orm import Session

from models.base import get_db
from models.schemas import AddressBookPageRequest, Response
from .service import AddressBookService

router = APIRouter()
service = AddressBookService()


@router.get("/address-book/select-page")
@router.post("/address-book/select-page")
async def select_page(
    request: Request,
    req: AddressBookPageRequest = None,
    db: Session = Depends(get_db)
):
    """地址簿分页查询"""
    if req is None:
        req = AddressBookPageRequest()
    return service.select_page(db, req)


@router.post("/address-book/insert")
async def insert(
    data: dict,
    db: Session = Depends(get_db)
):
    """地址簿新增"""
    try:
        result = service.insert(db, data)
        return result
    except Exception as e:
        return Response(code=1, status="error", message=str(e))


@router.post("/address-book/update")
async def update(
    data: dict,
    db: Session = Depends(get_db)
):
    """地址簿更新"""
    try:
        result = service.update(db, data)
        return result
    except ValueError as e:
        return Response(code=1, status="error", message=str(e))
    except Exception as e:
        return Response(code=1, status="error", message=str(e))


@router.delete("/address-book/delete/{id}")
async def delete(
    id: str,
    db: Session = Depends(get_db)
):
    """地址簿删除"""
    try:
        service.delete(db, id)
        return Response(message="删除成功")
    except ValueError as e:
        return Response(code=1, status="error", message=str(e))
    except Exception as e:
        return Response(code=1, status="error", message=str(e))
