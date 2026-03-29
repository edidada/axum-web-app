from sqlalchemy.orm import Session
from sqlalchemy import and_
from typing import Type, Any, List, Optional
from math import ceil

from models.schemas import PageResult


def paginate_query(
    db: Session,
    model: Type[Any],
    page: int,
    size: int,
    filters: Optional[List] = None,
    order_by: Any = None
) -> PageResult:
    """
    通用分页查询
    
    Args:
        db: 数据库会话
        model: 模型类
        page: 页码（从1开始）
        size: 每页大小
        filters: 过滤条件列表
        order_by: 排序条件
    
    Returns:
        PageResult: 分页结果
    """
    query = db.query(model)
    
    if filters:
        query = query.filter(and_(*filters))
    
    total = query.count()
    
    if order_by is not None:
        query = query.order_by(order_by)
    
    offset = (page - 1) * size
    records = query.offset(offset).limit(size).all()
    
    return PageResult(
        records=[r.to_dict() for r in records],
        total=total,
        size=size,
        current=page,
        pages=ceil(total / size) if size > 0 else 0
    )
