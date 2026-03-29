from sqlalchemy.orm import Session
from typing import Optional, List

from models.models import SysDict
from models.schemas import SysDictPageRequest, Response
from utils.id_generator import generate_id_with_prefix
from handlers.crud.common import paginate_query


class SysDictService:
    """字典服务类"""
    
    @staticmethod
    def select_page(db: Session, req: SysDictPageRequest) -> dict:
        """分页查询"""
        filters = []
        if req.code:
            filters.append(SysDict.code.like(f"%{req.code}%"))
        if req.name:
            filters.append(SysDict.name.like(f"%{req.name}%"))
        
        return paginate_query(
            db, SysDict, req.page, req.size,
            filters=filters if filters else None,
            order_by=SysDict.sort
        )
    
    @staticmethod
    def insert(db: Session, data: dict) -> dict:
        """新增"""
        dict_obj = SysDict(**data)
        dict_obj.id = generate_id_with_prefix("DICT")
        db.add(dict_obj)
        db.commit()
        db.refresh(dict_obj)
        return dict_obj.to_dict()
    
    @staticmethod
    def update(db: Session, data: dict) -> dict:
        """更新"""
        id = data.get("id")
        if not id:
            raise ValueError("缺少字典ID")
        
        dict_obj = db.query(SysDict).filter(SysDict.id == id).first()
        if not dict_obj:
            raise ValueError("字典不存在")
        
        for key, value in data.items():
            if hasattr(dict_obj, key) and key != "id":
                setattr(dict_obj, key, value)
        
        db.commit()
        db.refresh(dict_obj)
        return dict_obj.to_dict()
    
    @staticmethod
    def delete(db: Session, id: str) -> None:
        """删除"""
        dict_obj = db.query(SysDict).filter(SysDict.id == id).first()
        if not dict_obj:
            raise ValueError("字典不存在")
        
        db.delete(dict_obj)
        db.commit()
