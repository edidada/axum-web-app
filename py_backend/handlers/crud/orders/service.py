from sqlalchemy.orm import Session

from models.models import Orders
from models.schemas import OrdersPageRequest
from handlers.crud.common import paginate_query


class OrdersService:
    """订单服务类"""
    
    @staticmethod
    def select_page(db: Session, req: OrdersPageRequest) -> dict:
        """分页查询"""
        filters = []
        if req.number:
            filters.append(Orders.number.like(f"%{req.number}%"))
        if req.status:
            filters.append(Orders.status == req.status)
        if req.phone:
            filters.append(Orders.phone.like(f"%{req.phone}%"))
        if req.consignee:
            filters.append(Orders.consignee.like(f"%{req.consignee}%"))
        
        return paginate_query(
            db, Orders, req.page, req.size,
            filters=filters if filters else None,
            order_by=Orders.order_time.desc()
        )
