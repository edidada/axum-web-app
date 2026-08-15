from sqlalchemy.orm import Session

from models.models import Employee
from models.schemas import EmployeePageRequest
from utils.id_generator import generate_id_with_prefix
from handlers.crud.common import paginate_query


class EmployeeService:
    """员工服务类"""
    
    @staticmethod
    def select_page(db: Session, req: EmployeePageRequest) -> dict:
        """分页查询"""
        filters = [Employee.is_deleted == 0]
        if req.name:
            filters.append(Employee.name.like(f"%{req.name}%"))
        if req.username:
            filters.append(Employee.username.like(f"%{req.username}%"))
        
        return paginate_query(
            db, Employee, req.page, req.size,
            filters=filters,
            order_by=Employee.create_time.desc()
        )
    
    @staticmethod
    def insert(db: Session, data: dict) -> dict:
        """新增"""
        emp = Employee(**data)
        emp.id = generate_id_with_prefix("EMP")
        emp.is_deleted = 0
        db.add(emp)
        db.commit()
        db.refresh(emp)
        return emp.to_dict()
    
    @staticmethod
    def update(db: Session, data: dict) -> dict:
        """更新"""
        id = data.get("id")
        if not id:
            raise ValueError("缺少员工ID")
        
        emp = db.query(Employee).filter(Employee.id == id).first()
        if not emp:
            raise ValueError("员工不存在")
        
        for key, value in data.items():
            if hasattr(emp, key) and key != "id":
                setattr(emp, key, value)
        
        db.commit()
        db.refresh(emp)
        return emp.to_dict()
    
    @staticmethod
    def delete(db: Session, id: str) -> None:
        """删除（软删除）"""
        emp = db.query(Employee).filter(Employee.id == id).first()
        if not emp:
            raise ValueError("员工不存在")
        
        emp.is_deleted = 1
        db.commit()
