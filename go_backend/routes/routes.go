package routes

import (
	"go_backend/handlers"
	"go_backend/middleware"

	"github.com/gin-gonic/gin"
)

// SetupRoutes 设置路由
func SetupRoutes(r *gin.Engine) {
	// 跨域中间件
	r.Use(middleware.CORSMiddleware())

	// 健康检查路由（不需要认证）
	r.GET("/health", handlers.HealthCheck)
	r.GET("/health/detailed", handlers.HealthCheckDetailed)
	r.GET("/ready", handlers.ReadinessCheck)
	r.GET("/live", handlers.LivenessCheck)

	// 认证路由（不需要认证）
	r.POST("/api/login", handlers.Login)

	// 需要认证的路由组
	auth := r.Group("/")
	auth.Use(middleware.AuthMiddleware())
	{
		// 登出
		auth.POST("/api/logout", handlers.Logout)

		// SysDict 字典管理
		auth.GET("/sys-dict/select-page", handlers.SysDictSelectPage)
		auth.POST("/sys-dict/select-page", handlers.SysDictSelectPage)
		auth.POST("/sys-dict/insert", handlers.SysDictInsert)
		auth.POST("/sys-dict/update", handlers.SysDictUpdate)
		auth.DELETE("/sys-dict/delete/:id", handlers.SysDictDelete)

		// AddressBook 地址簿
		auth.GET("/address-book/select-page", handlers.AddressBookSelectPage)
		auth.POST("/address-book/select-page", handlers.AddressBookSelectPage)
		auth.POST("/address-book/insert", handlers.AddressBookInsert)
		auth.POST("/address-book/update", handlers.AddressBookUpdate)
		auth.DELETE("/address-book/delete/:id", handlers.AddressBookDelete)

		// Category 分类管理
		auth.GET("/category/select-page", handlers.CategorySelectPage)
		auth.POST("/category/select-page", handlers.CategorySelectPage)
		auth.POST("/category/insert", handlers.CategoryInsert)
		auth.POST("/category/update", handlers.CategoryUpdate)
		auth.DELETE("/category/delete/:id", handlers.CategoryDelete)

		// Dish 菜品管理
		auth.GET("/dish/select-page", handlers.DishSelectPage)
		auth.POST("/dish/select-page", handlers.DishSelectPage)
		auth.POST("/dish/insert", handlers.DishInsert)
		auth.POST("/dish/update", handlers.DishUpdate)
		auth.DELETE("/dish/delete/:id", handlers.DishDelete)

		// Employee 员工管理
		auth.GET("/employee/select-page", handlers.EmployeeSelectPage)
		auth.POST("/employee/select-page", handlers.EmployeeSelectPage)
		auth.POST("/employee/insert", handlers.EmployeeInsert)
		auth.POST("/employee/update", handlers.EmployeeUpdate)
		auth.DELETE("/employee/delete/:id", handlers.EmployeeDelete)

		// Orders 订单管理
		auth.GET("/orders/select-page", handlers.OrdersSelectPage)
		auth.POST("/orders/select-page", handlers.OrdersSelectPage)

		// Setmeal 套餐管理
		auth.GET("/setmeal/select-page", handlers.SetmealSelectPage)
		auth.POST("/setmeal/select-page", handlers.SetmealSelectPage)

		// User 用户管理
		auth.GET("/user/select-page", handlers.UserSelectPage)
		auth.POST("/user/select-page", handlers.UserSelectPage)

		// 图片管理
		auth.POST("/image/upload-from-url", handlers.UploadImageFromURL)
		auth.DELETE("/image/:objectName", handlers.DeleteImage)
		auth.GET("/image/url/:objectName", handlers.GetImageURL)
	}
}
