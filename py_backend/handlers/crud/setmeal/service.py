from sqlalchemy.orm import Session

from models.models import Setmeal
from models.schemas import SetmealPageRequest
from handlers.crud.common import paginate_query


class SetmealService:
    """套餐服务类"""
    
    @staticmethod
    def select_page(db: Session, req: SetmealPageRequest) -> dict:
        """分页查询"""
        filters = [Setmeal.is_deleted == 0]
        if req.name:
            filters.append(Setmeal.name.like(f"%{req.name}%"))
        if req.category_id:
            filters.append(Setmeal.category_id == req.category_id)
        if req.status:
            filters.append(Setmeal.status == req.status)
        
        return paginate_query(
            db, Setmeal, req.page, req.size,
            filters=filters,
            order_by=Setmeal.create_time.desc()
        )
