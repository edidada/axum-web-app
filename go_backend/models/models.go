package models

import (
	"time"
)

// SysDict 字典表
type SysDict struct {
	ID         string     `gorm:"column:id;primaryKey" json:"id"`
	Name       string     `gorm:"column:name" json:"name"`
	Code       string     `gorm:"column:code" json:"code"`
	Value      string     `gorm:"column:value" json:"value"`
	Sort       int        `gorm:"column:sort" json:"sort"`
	Remark     string     `gorm:"column:remark" json:"remark"`
	State      int        `gorm:"column:state" json:"state"`
	CreateDate *time.Time `gorm:"column:create_date" json:"create_date"`
}

func (SysDict) TableName() string {
	return "sys_dict"
}

// AddressBook 地址簿
type AddressBook struct {
	ID           string     `gorm:"column:id;primaryKey" json:"id"`
	UserID       string     `gorm:"column:user_id" json:"user_id"`
	Consignee    string     `gorm:"column:consignee" json:"consignee"`
	Sex          string     `gorm:"column:sex" json:"sex"`
	Phone        string     `gorm:"column:phone" json:"phone"`
	ProvinceCode string     `gorm:"column:province_code" json:"province_code"`
	ProvinceName string     `gorm:"column:province_name" json:"province_name"`
	CityCode     string     `gorm:"column:city_code" json:"city_code"`
	CityName     string     `gorm:"column:city_name" json:"city_name"`
	DistrictCode string     `gorm:"column:district_code" json:"district_code"`
	DistrictName string     `gorm:"column:district_name" json:"district_name"`
	Detail       string     `gorm:"column:detail" json:"detail"`
	Label        string     `gorm:"column:label" json:"label"`
	IsDefault    int        `gorm:"column:is_default" json:"is_default"`
	IsDeleted    int        `gorm:"column:is_deleted" json:"is_deleted"`
	CreateTime   *time.Time `gorm:"column:create_time" json:"create_time"`
	UpdateTime   *time.Time `gorm:"column:update_time" json:"update_time"`
	CreateUser   string     `gorm:"column:create_user" json:"create_user"`
	UpdateUser   string     `gorm:"column:update_user" json:"update_user"`
}

func (AddressBook) TableName() string {
	return "address_book"
}

// Category 分类
type Category struct {
	ID         string     `gorm:"column:id;primaryKey" json:"id"`
	Type       int        `gorm:"column:type" json:"type"`
	Name       string     `gorm:"column:name" json:"name"`
	Sort       int        `gorm:"column:sort" json:"sort"`
	IsDeleted  int        `gorm:"column:is_deleted" json:"is_deleted"`
	CreateTime *time.Time `gorm:"column:create_time" json:"create_time"`
	UpdateTime *time.Time `gorm:"column:update_time" json:"update_time"`
	CreateUser string     `gorm:"column:create_user" json:"create_user"`
	UpdateUser string     `gorm:"column:update_user" json:"update_user"`
}

func (Category) TableName() string {
	return "category"
}

// Dish 菜品
type Dish struct {
	ID          string     `gorm:"column:id;primaryKey" json:"id"`
	Name        string     `gorm:"column:name" json:"name"`
	CategoryID  string     `gorm:"column:category_id" json:"category_id"`
	Price       float64    `gorm:"column:price" json:"price"`
	Code        string     `gorm:"column:code" json:"code"`
	Image       string     `gorm:"column:image" json:"image"`
	Description string     `gorm:"column:description" json:"description"`
	Status      int        `gorm:"column:status" json:"status"`
	Sort        int        `gorm:"column:sort" json:"sort"`
	IsDeleted   int        `gorm:"column:is_deleted" json:"is_deleted"`
	CreateTime  *time.Time `gorm:"column:create_time" json:"create_time"`
	UpdateTime  *time.Time `gorm:"column:update_time" json:"update_time"`
	CreateUser  string     `gorm:"column:create_user" json:"create_user"`
	UpdateUser  string     `gorm:"column:update_user" json:"update_user"`
}

func (Dish) TableName() string {
	return "dish"
}

// DishFlavor 菜品口味
type DishFlavor struct {
	ID     string `gorm:"column:id;primaryKey" json:"id"`
	DishID string `gorm:"column:dish_id" json:"dish_id"`
	Name   string `gorm:"column:name" json:"name"`
	Value  string `gorm:"column:value" json:"value"`
}

func (DishFlavor) TableName() string {
	return "dish_flavor"
}

// Employee 员工
type Employee struct {
	ID         string     `gorm:"column:id;primaryKey" json:"id"`
	Name       string     `gorm:"column:name" json:"name"`
	Username   string     `gorm:"column:username" json:"username"`
	Password   string     `gorm:"column:password" json:"password"`
	Phone      string     `gorm:"column:phone" json:"phone"`
	Sex        string     `gorm:"column:sex" json:"sex"`
	IDNumber   string     `gorm:"column:id_number" json:"id_number"`
	Status     int        `gorm:"column:status" json:"status"`
	IsDeleted  int        `gorm:"column:is_deleted" json:"is_deleted"`
	CreateTime *time.Time `gorm:"column:create_time" json:"create_time"`
	UpdateTime *time.Time `gorm:"column:update_time" json:"update_time"`
	CreateUser string     `gorm:"column:create_user" json:"create_user"`
	UpdateUser string     `gorm:"column:update_user" json:"update_user"`
}

func (Employee) TableName() string {
	return "employee"
}

// OrderDetail 订单详情
type OrderDetail struct {
	ID         string  `gorm:"column:id;primaryKey" json:"id"`
	Name       string  `gorm:"column:name" json:"name"`
	Image      string  `gorm:"column:image" json:"image"`
	OrderID    string  `gorm:"column:order_id" json:"order_id"`
	DishID     string  `gorm:"column:dish_id" json:"dish_id"`
	SetmealID  string  `gorm:"column:setmeal_id" json:"setmeal_id"`
	DishFlavor string  `gorm:"column:dish_flavor" json:"dish_flavor"`
	Number     int     `gorm:"column:number" json:"number"`
	Amount     float64 `gorm:"column:amount" json:"amount"`
}

func (OrderDetail) TableName() string {
	return "order_detail"
}

// Orders 订单
type Orders struct {
	ID                    string     `gorm:"column:id;primaryKey" json:"id"`
	Number                string     `gorm:"column:number" json:"number"`
	Status                int        `gorm:"column:status" json:"status"`
	UserID                string     `gorm:"column:user_id" json:"user_id"`
	AddressBookID         string     `gorm:"column:address_book_id" json:"address_book_id"`
	OrderTime             *time.Time `gorm:"column:order_time" json:"order_time"`
	CheckoutTime          *time.Time `gorm:"column:checkout_time" json:"checkout_time"`
	PayMethod             int        `gorm:"column:pay_method" json:"pay_method"`
	PayStatus             int        `gorm:"column:pay_status" json:"pay_status"`
	Amount                float64    `gorm:"column:amount" json:"amount"`
	Remark                string     `gorm:"column:remark" json:"remark"`
	Phone                 string     `gorm:"column:phone" json:"phone"`
	Address               string     `gorm:"column:address" json:"address"`
	UserName              string     `gorm:"column:user_name" json:"user_name"`
	Consignee             string     `gorm:"column:consignee" json:"consignee"`
	CancelReason          string     `gorm:"column:cancel_reason" json:"cancel_reason"`
	RejectionReason       string     `gorm:"column:rejection_reason" json:"rejection_reason"`
	CancelTime            *time.Time `gorm:"column:cancel_time" json:"cancel_time"`
	EstimatedDeliveryTime *time.Time `gorm:"column:estimated_delivery_time" json:"estimated_delivery_time"`
	DeliveryStatus        int        `gorm:"column:delivery_status" json:"delivery_status"`
	DeliveryTime          *time.Time `gorm:"column:delivery_time" json:"delivery_time"`
	PackAmount            int        `gorm:"column:pack_amount" json:"pack_amount"`
	TablewareNumber       int        `gorm:"column:tableware_number" json:"tableware_number"`
	TablewareStatus       int        `gorm:"column:tableware_status" json:"tableware_status"`
}

func (Orders) TableName() string {
	return "orders"
}

// Setmeal 套餐
type Setmeal struct {
	ID          string     `gorm:"column:id;primaryKey" json:"id"`
	CategoryID  string     `gorm:"column:category_id" json:"category_id"`
	Name        string     `gorm:"column:name" json:"name"`
	Price       float64    `gorm:"column:price" json:"price"`
	Status      int        `gorm:"column:status" json:"status"`
	Code        string     `gorm:"column:code" json:"code"`
	Description string     `gorm:"column:description" json:"description"`
	Image       string     `gorm:"column:image" json:"image"`
	IsDeleted   int        `gorm:"column:is_deleted" json:"is_deleted"`
	CreateTime  *time.Time `gorm:"column:create_time" json:"create_time"`
	UpdateTime  *time.Time `gorm:"column:update_time" json:"update_time"`
	CreateUser  string     `gorm:"column:create_user" json:"create_user"`
	UpdateUser  string     `gorm:"column:update_user" json:"update_user"`
}

func (Setmeal) TableName() string {
	return "setmeal"
}

// SetmealDish 套餐菜品
type SetmealDish struct {
	ID        string  `gorm:"column:id;primaryKey" json:"id"`
	SetmealID string  `gorm:"column:setmeal_id" json:"setmeal_id"`
	DishID    string  `gorm:"column:dish_id" json:"dish_id"`
	Name      string  `gorm:"column:name" json:"name"`
	Price     float64 `gorm:"column:price" json:"price"`
	Copies    int     `gorm:"column:copies" json:"copies"`
}

func (SetmealDish) TableName() string {
	return "setmeal_dish"
}

// ShoppingCart 购物车
type ShoppingCart struct {
	ID         string     `gorm:"column:id;primaryKey" json:"id"`
	Name       string     `gorm:"column:name" json:"name"`
	Image      string     `gorm:"column:image" json:"image"`
	UserID     string     `gorm:"column:user_id" json:"user_id"`
	DishID     string     `gorm:"column:dish_id" json:"dish_id"`
	SetmealID  string     `gorm:"column:setmeal_id" json:"setmeal_id"`
	DishFlavor string     `gorm:"column:dish_flavor" json:"dish_flavor"`
	Number     int        `gorm:"column:number" json:"number"`
	Amount     float64    `gorm:"column:amount" json:"amount"`
	CreateTime *time.Time `gorm:"column:create_time" json:"create_time"`
}

func (ShoppingCart) TableName() string {
	return "shopping_cart"
}

// User 用户
type User struct {
	ID         string     `gorm:"column:id;primaryKey" json:"id"`
	OpenID     string     `gorm:"column:openid" json:"openid"`
	Name       string     `gorm:"column:name" json:"name"`
	Phone      string     `gorm:"column:phone" json:"phone"`
	Sex        string     `gorm:"column:sex" json:"sex"`
	IDNumber   string     `gorm:"column:id_number" json:"id_number"`
	Avatar     string     `gorm:"column:avatar" json:"avatar"`
	Status     int        `gorm:"column:status" json:"status"`
	IsDeleted  int        `gorm:"column:is_deleted" json:"is_deleted"`
	CreateTime *time.Time `gorm:"column:create_time" json:"create_time"`
}

func (User) TableName() string {
	return "user"
}
