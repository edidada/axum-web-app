from sqlalchemy.orm import Session
from typing import Optional, List

from models.models import AddressBook
from models.schemas import AddressBookPageRequest
from utils.id_generator import generate_id_with_prefix
from handlers.crud.common import paginate_query


class AddressBookService:
    """地址簿服务类"""
    
    @staticmethod
    def select_page(db: Session, req: AddressBookPageRequest) -> dict:
        """分页查询"""
        filters = [AddressBook.is_deleted == 0]
        if req.consignee:
            filters.append(AddressBook.consignee.like(f"%{req.consignee}%"))
        if req.phone:
            filters.append(AddressBook.phone.like(f"%{req.phone}%"))
        
        return paginate_query(
            db, AddressBook, req.page, req.size,
            filters=filters,
            order_by=AddressBook.create_time.desc()
        )
    
    @staticmethod
    def insert(db: Session, data: dict) -> dict:
        """新增"""
        addr = AddressBook(**data)
        addr.id = generate_id_with_prefix("ADDR")
        addr.is_deleted = 0
        db.add(addr)
        db.commit()
        db.refresh(addr)
        return addr.to_dict()
    
    @staticmethod
    def update(db: Session, data: dict) -> dict:
        """更新"""
        id = data.get("id")
        if not id:
            raise ValueError("缺少地址ID")
        
        addr = db.query(AddressBook).filter(AddressBook.id == id).first()
        if not addr:
            raise ValueError("地址不存在")
        
        for key, value in data.items():
            if hasattr(addr, key) and key != "id":
                setattr(addr, key, value)
        
        db.commit()
        db.refresh(addr)
        return addr.to_dict()
    
    @staticmethod
    def delete(db: Session, id: str) -> None:
        """删除（软删除）"""
        addr = db.query(AddressBook).filter(AddressBook.id == id).first()
        if not addr:
            raise ValueError("地址不存在")
        
        addr.is_deleted = 1
        db.commit()
