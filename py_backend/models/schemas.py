from pydantic import BaseModel, Field
from typing import Optional, List, Any
from datetime import datetime


class Response(BaseModel):
    """通用响应"""
    code: int = 0
    status: str = "success"
    message: Optional[str] = None
    data: Optional[Any] = None


class PageResult(BaseModel):
    """分页结果"""
    records: List[Any]
    total: int
    size: int
    current: int
    pages: int


# SysDict 相关
class SysDictPageRequest(BaseModel):
    page: int = 1
    size: int = 10
    code: Optional[str] = None
    name: Optional[str] = None


# AddressBook 相关
class AddressBookPageRequest(BaseModel):
    page: int = 1
    size: int = 10
    consignee: Optional[str] = None
    phone: Optional[str] = None


# Category 相关
class CategoryPageRequest(BaseModel):
    page: int = 1
    size: int = 10
    name: Optional[str] = None
    type: Optional[int] = None


# Dish 相关
class DishPageRequest(BaseModel):
    page: int = 1
    size: int = 10
    name: Optional[str] = None
    category_id: Optional[str] = None
    status: Optional[int] = None


# Employee 相关
class EmployeePageRequest(BaseModel):
    page: int = 1
    size: int = 10
    name: Optional[str] = None
    username: Optional[str] = None


# Orders 相关
class OrdersPageRequest(BaseModel):
    page: int = 1
    size: int = 10
    number: Optional[str] = None
    status: Optional[int] = None
    phone: Optional[str] = None
    consignee: Optional[str] = None


# Setmeal 相关
class SetmealPageRequest(BaseModel):
    page: int = 1
    size: int = 10
    name: Optional[str] = None
    category_id: Optional[str] = None
    status: Optional[int] = None


# User 相关
class UserPageRequest(BaseModel):
    page: int = 1
    size: int = 10
    name: Optional[str] = None
    phone: Optional[str] = None


# 登录相关
class LoginRequest(BaseModel):
    username: str
    password: str


class LoginResponse(BaseModel):
    code: int = 0
    status: str = "success"
    data: Optional[dict] = None


# 图片上传
class ImageUploadRequest(BaseModel):
    url: str
