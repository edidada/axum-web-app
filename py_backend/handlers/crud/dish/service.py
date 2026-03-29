from sqlalchemy.orm import Session

from models.models import Dish
from models.schemas import DishPageRequest
from utils.id_generator import generate_id_with_prefix
from handlers.crud.common import paginate_query


class DishService:
    """菜品服务类"""
    
    @staticmethod
    def select_page(db: Session, req: DishPageRequest) -> dict:
        """分页查询"""
        filters = [Dish.is_deleted == 0]
        if req.name:
            filters.append(Dish.name.like(f"%{req.name}%"))
        if req.category_id:
            filters.append(Dish.category_id == req.category_id)
        if req.status:
            filters.append(Dish.status == req.status)
        
        return paginate_query(
            db, Dish, req.page, req.size,
            filters=filters,
            order_by=Dish.create_time.desc()
        )
    
    @staticmethod
    def insert(db: Session, data: dict) -> dict:
        """新增"""
        dish = Dish(**data)
        dish.id = generate_id_with_prefix("DISH")
        dish.is_deleted = 0
        db.add(dish)
        db.commit()
        db.refresh(dish)
        return dish.to_dict()
    
    @staticmethod
    def update(db: Session, data: dict) -> dict:
        """更新"""
        id = data.get("id")
        if not id:
            raise ValueError("缺少菜品ID")
        
        dish = db.query(Dish).filter(Dish.id == id).first()
        if not dish:
            raise ValueError("菜品不存在")
        
        for key, value in data.items():
            if hasattr(dish, key) and key != "id":
                setattr(dish, key, value)
        
        db.commit()
        db.refresh(dish)
        return dish.to_dict()
    
    @staticmethod
    def delete(db: Session, id: str) -> None:
        """删除（软删除）"""
        dish = db.query(Dish).filter(Dish.id == id).first()
        if not dish:
            raise ValueError("菜品不存在")
        
        dish.is_deleted = 1
        db.commit()
