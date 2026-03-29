package models

// PageQuery 通用分页请求
type PageQuery struct {
	Page     int `json:"page" form:"page"`
	PageSize int `json:"pageSize" form:"pageSize"`
}

// PageResult 通用分页响应
type PageResult struct {
	Records     interface{} `json:"records"`
	Total       int64       `json:"total"`
	Size        int         `json:"size"`
	Current     int         `json:"current"`
	Pages       int         `json:"pages"`
}

// Response 通用响应
type Response struct {
	Code    int         `json:"code"`
	Status  string      `json:"status"`
	Message string      `json:"message,omitempty"`
	Data    interface{} `json:"data,omitempty"`
}

// LoginRequest 登录请求
type LoginRequest struct {
	Username string `json:"username" binding:"required"`
	Password string `json:"password" binding:"required"`
}

// LoginResponse 登录响应
type LoginResponse struct {
	Code      int    `json:"code"`
	Status    string `json:"status"`
	SessionID string `json:"session_id,omitempty"`
	Message   string `json:"message,omitempty"`
}

// SysDictPageRequest 字典分页请求
type SysDictPageRequest struct {
	Page int    `json:"page" form:"page"`
	Size int    `json:"size" form:"size"`
	Code string `json:"code" form:"code"`
	Name string `json:"name" form:"name"`
}

// AddressBookPageRequest 地址簿分页请求
type AddressBookPageRequest struct {
	Page      int    `json:"page" form:"page"`
	Size      int    `json:"size" form:"size"`
	Consignee string `json:"consignee" form:"consignee"`
	Phone     string `json:"phone" form:"phone"`
}

// CategoryPageRequest 分类分页请求
type CategoryPageRequest struct {
	Page int    `json:"page" form:"page"`
	Size int    `json:"size" form:"size"`
	Name string `json:"name" form:"name"`
	Type int    `json:"type" form:"type"`
}

// DishPageRequest 菜品分页请求
type DishPageRequest struct {
	Page       int    `json:"page" form:"page"`
	Size       int    `json:"size" form:"size"`
	Name       string `json:"name" form:"name"`
	CategoryID string `json:"categoryId" form:"categoryId"`
	Status     int    `json:"status" form:"status"`
}

// DishFlavorPageRequest 菜品口味分页请求
type DishFlavorPageRequest struct {
	Page   int    `json:"page" form:"page"`
	Size   int    `json:"size" form:"size"`
	DishID string `json:"dishId" form:"dishId"`
	Name   string `json:"name" form:"name"`
}

// EmployeePageRequest 员工分页请求
type EmployeePageRequest struct {
	Page     int    `json:"page" form:"page"`
	Size     int    `json:"size" form:"size"`
	Name     string `json:"name" form:"name"`
	Username string `json:"username" form:"username"`
}

// OrderDetailPageRequest 订单详情分页请求
type OrderDetailPageRequest struct {
	Page    int    `json:"page" form:"page"`
	Size    int    `json:"size" form:"size"`
	OrderID string `json:"orderId" form:"orderId"`
	Name    string `json:"name" form:"name"`
}

// OrdersPageRequest 订单分页请求
type OrdersPageRequest struct {
	Page      int    `json:"page" form:"page"`
	Size      int    `json:"size" form:"size"`
	Number    string `json:"number" form:"number"`
	Status    int    `json:"status" form:"status"`
	Phone     string `json:"phone" form:"phone"`
	Consignee string `json:"consignee" form:"consignee"`
}

// SetmealPageRequest 套餐分页请求
type SetmealPageRequest struct {
	Page       int    `json:"page" form:"page"`
	Size       int    `json:"size" form:"size"`
	Name       string `json:"name" form:"name"`
	CategoryID string `json:"categoryId" form:"categoryId"`
	Status     int    `json:"status" form:"status"`
}

// SetmealDishPageRequest 套餐菜品分页请求
type SetmealDishPageRequest struct {
	Page      int    `json:"page" form:"page"`
	Size      int    `json:"size" form:"size"`
	SetmealID string `json:"setmealId" form:"setmealId"`
	DishID    string `json:"dishId" form:"dishId"`
}

// ShoppingCartPageRequest 购物车分页请求
type ShoppingCartPageRequest struct {
	Page   int    `json:"page" form:"page"`
	Size   int    `json:"size" form:"size"`
	UserID string `json:"userId" form:"userId"`
	DishID string `json:"dishId" form:"dishId"`
}

// UserPageRequest 用户分页请求
type UserPageRequest struct {
	Page  int    `json:"page" form:"page"`
	Size  int    `json:"size" form:"size"`
	Name  string `json:"name" form:"name"`
	Phone string `json:"phone" form:"phone"`
}

// UploadImageRequest 上传图片请求
type UploadImageRequest struct {
	ImageURL string `json:"image_url" binding:"required"`
	ObjectName string `json:"object_name,omitempty"`
}

// ImageSyncRequest 图片同步请求
type ImageSyncRequest struct {
	TableName string `json:"table_name" binding:"required"`
	FieldName string `json:"field_name" binding:"required"`
}
