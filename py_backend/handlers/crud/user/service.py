from sqlalchemy.orm import Session

from models.models import User
from models.schemas import UserPageRequest
from handlers.crud.common import paginate_query


class UserService:
    """用户服务类"""
    
    @staticmethod
    def select_page(db: Session, req: UserPageRequest) -> dict:
        """分页查询"""
        filters = [User.is_deleted == 0]
        if req.name:
            filters.append(User.name.like(f"%{req.name}%"))
        if req.phone:
            filters.append(User.phone.like(f"%{req.phone}%"))
        
        return paginate_query(
            db, User, req.page, req.size,
            filters=filters,
            order_by=User.create_time.desc()
        )
