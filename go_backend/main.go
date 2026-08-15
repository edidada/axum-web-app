package main

import (
	"fmt"
	"log"

	"go_backend/config"
	"go_backend/routes"
	"go_backend/utils"

	"github.com/gin-gonic/gin"
)

func main() {
	// 加载配置
	cfg, err := config.LoadConfig("config/config.yaml")
	if err != nil {
		log.Printf("Warning: Failed to load config file, using defaults: %v", err)
		cfg = config.GetConfig()
	}

	fmt.Println("=====================================")
	fmt.Println("🚀 Go Backend Server Starting...")
	fmt.Println("=====================================")

	// 初始化数据库
	utils.InitDB()

	// 初始化 Redis
	utils.InitRedis()

	// 初始化 MinIO
	utils.InitMinio()

	// 设置 Gin 模式
	gin.SetMode(gin.DebugMode)

	// 创建路由
	r := gin.Default()

	// 设置路由
	routes.SetupRoutes(r)

	// 启动服务器
	addr := fmt.Sprintf("127.0.0.1:%d", cfg.Webapp.Port)
	fmt.Printf("\n🌐 Server listening on http://%s\n", addr)
	fmt.Println("=====================================")

	if err := r.Run(addr); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}
