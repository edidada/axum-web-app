from fastapi import APIRouter, Depends, Request
from sqlalchemy.orm import Session

from models.base import get_db
from models.schemas import EmployeePageRequest, Response
from .service import EmployeeService

router = APIRouter()
service = EmployeeService()


@router.get("/employee/select-page")
@router.post("/employee/select-page")
async def select_page(
    request: Request,
    req: EmployeePageRequest = None,
    db: Session = Depends(get_db)
):
    """员工分页查询"""
    if req is None:
        req = EmployeePageRequest()
    return service.select_page(db, req)


@router.post("/employee/insert")
async def insert(
    data: dict,
    db: Session = Depends(get_db)
):
    """员工新增"""
    try:
        result = service.insert(db, data)
        return result
    except Exception as e:
        return Response(code=1, status="error", message=str(e))


@router.post("/employee/update")
async def update(
    data: dict,
    db: Session = Depends(get_db)
):
    """员工更新"""
    try:
        result = service.update(db, data)
        return result
    except ValueError as e:
        return Response(code=1, status="error", message=str(e))
    except Exception as e:
        return Response(code=1, status="error", message=str(e))


@router.delete("/employee/delete/{id}")
async def delete(
    id: str,
    db: Session = Depends(get_db)
):
    """员工删除"""
    try:
        service.delete(db, id)
        return Response(message="删除成功")
    except ValueError as e:
        return Response(code=1, status="error", message=str(e))
    except Exception as e:
        return Response(code=1, status="error", message=str(e))
