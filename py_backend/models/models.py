from sqlalchemy import Column, String, Integer, DateTime, Numeric, Text, ForeignKey
from datetime import datetime
from models.base import BaseModel


class SysDict(BaseModel):
    """字典表"""
    __tablename__ = "sys_dict"
    
    id = Column(String(64), primary_key=True)
    code = Column(String(50))
    name = Column(String(100))
    value = Column(String(200))
    sort = Column(Integer, default=0)
    remark = Column(String(500))
    create_date = Column(DateTime, default=datetime.now)


class AddressBook(BaseModel):
    """地址簿"""
    __tablename__ = "address_book"
    
    id = Column(String(64), primary_key=True)
    user_id = Column(String(64))
    consignee = Column(String(100))
    sex = Column(String(10))
    phone = Column(String(20))
    province_code = Column(String(20))
    province_name = Column(String(100))
    city_code = Column(String(20))
    city_name = Column(String(100))
    district_code = Column(String(20))
    district_name = Column(String(100))
    detail = Column(String(500))
    label = Column(String(50))
    is_default = Column(Integer, default=0)
    is_deleted = Column(Integer, default=0)
    create_time = Column(DateTime, default=datetime.now)
    update_time = Column(DateTime, default=datetime.now, onupdate=datetime.now)


class Category(BaseModel):
    """分类"""
    __tablename__ = "category"
    
    id = Column(String(64), primary_key=True)
    type = Column(Integer)
    name = Column(String(100))
    sort = Column(Integer, default=0)
    is_deleted = Column(Integer, default=0)
    create_time = Column(DateTime, default=datetime.now)
    update_time = Column(DateTime, default=datetime.now, onupdate=datetime.now)


class Dish(BaseModel):
    """菜品"""
    __tablename__ = "dish"
    
    id = Column(String(64), primary_key=True)
    name = Column(String(100))
    category_id = Column(String(64))
    price = Column(Numeric(10, 2))
    image = Column(String(500))
    description = Column(Text)
    status = Column(Integer, default=1)
    is_deleted = Column(Integer, default=0)
    create_time = Column(DateTime, default=datetime.now)
    update_time = Column(DateTime, default=datetime.now, onupdate=datetime.now)


class Employee(BaseModel):
    """员工"""
    __tablename__ = "employee"
    
    id = Column(String(64), primary_key=True)
    name = Column(String(100))
    username = Column(String(50), unique=True)
    password = Column(String(100))
    phone = Column(String(20))
    sex = Column(String(10))
    id_number = Column(String(20))
    status = Column(Integer, default=1)
    is_deleted = Column(Integer, default=0)
    create_time = Column(DateTime, default=datetime.now)
    update_time = Column(DateTime, default=datetime.now, onupdate=datetime.now)


class Orders(BaseModel):
    """订单"""
    __tablename__ = "orders"
    
    id = Column(String(64), primary_key=True)
    number = Column(String(50))
    status = Column(Integer, default=0)
    user_id = Column(String(64))
    address_book_id = Column(String(64))
    order_time = Column(DateTime, default=datetime.now)
    checkout_time = Column(DateTime)
    pay_method = Column(Integer)
    pay_status = Column(Integer, default=0)
    amount = Column(Numeric(10, 2))
    remark = Column(String(500))
    phone = Column(String(20))
    address = Column(String(500))
    consignee = Column(String(100))


class Setmeal(BaseModel):
    """套餐"""
    __tablename__ = "setmeal"
    
    id = Column(String(64), primary_key=True)
    category_id = Column(String(64))
    name = Column(String(100))
    price = Column(Numeric(10, 2))
    status = Column(Integer, default=1)
    description = Column(Text)
    image = Column(String(500))
    is_deleted = Column(Integer, default=0)
    create_time = Column(DateTime, default=datetime.now)
    update_time = Column(DateTime, default=datetime.now, onupdate=datetime.now)


class User(BaseModel):
    """用户"""
    __tablename__ = "user"
    
    id = Column(String(64), primary_key=True)
    openid = Column(String(100))
    name = Column(String(100))
    phone = Column(String(20))
    sex = Column(String(10))
    id_number = Column(String(20))
    avatar = Column(String(500))
    is_deleted = Column(Integer, default=0)
    create_time = Column(DateTime, default=datetime.now)
