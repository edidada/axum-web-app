from sqlalchemy.orm import Session

from models.models import Category
from models.schemas import CategoryPageRequest
from utils.id_generator import generate_id_with_prefix
from handlers.crud.common import paginate_query


class CategoryService:
    """分类服务类"""
    
    @staticmethod
    def select_page(db: Session, req: CategoryPageRequest) -> dict:
        """分页查询"""
        filters = [Category.is_deleted == 0]
        if req.name:
            filters.append(Category.name.like(f"%{req.name}%"))
        if req.type:
            filters.append(Category.type == req.type)
        
        return paginate_query(
            db, Category, req.page, req.size,
            filters=filters,
            order_by=Category.sort
        )
    
    @staticmethod
    def insert(db: Session, data: dict) -> dict:
        """新增"""
        cat = Category(**data)
        cat.id = generate_id_with_prefix("CAT")
        cat.is_deleted = 0
        db.add(cat)
        db.commit()
        db.refresh(cat)
        return cat.to_dict()
    
    @staticmethod
    def update(db: Session, data: dict) -> dict:
        """更新"""
        id = data.get("id")
        if not id:
            raise ValueError("缺少分类ID")
        
        cat = db.query(Category).filter(Category.id == id).first()
        if not cat:
            raise ValueError("分类不存在")
        
        for key, value in data.items():
            if hasattr(cat, key) and key != "id":
                setattr(cat, key, value)
        
        db.commit()
        db.refresh(cat)
        return cat.to_dict()
    
    @staticmethod
    def delete(db: Session, id: str) -> None:
        """删除（软删除）"""
        cat = db.query(Category).filter(Category.id == id).first()
        if not cat:
            raise ValueError("分类不存在")
        
        cat.is_deleted = 1
        db.commit()
