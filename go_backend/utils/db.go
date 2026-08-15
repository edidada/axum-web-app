package utils

import (
	"fmt"
	"log"

	"go_backend/config"

	"gorm.io/driver/mysql"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

var DB *gorm.DB

func InitDB() *gorm.DB {
	cfg := config.GetConfig()

	// 打印 DSN 用于调试
	dsn := cfg.GetDatabaseDSN()
	fmt.Printf("Database DSN: %s\n", dsn)
	fmt.Printf("Database Type: %s\n", cfg.Database.DBType)
	fmt.Printf("Database Name: %s\n", cfg.Database.Database)

	var dialector gorm.Dialector

	if cfg.IsPostgreSQL() {
		dialector = postgres.Open(dsn)
	} else {
		dialector = mysql.Open(dsn)
	}

	db, err := gorm.Open(dialector, &gorm.Config{
		Logger: logger.Default.LogMode(logger.Info),
	})

	if err != nil {
		log.Fatalf("Failed to connect to database: %v", err)
	}

	DB = db
	fmt.Println("✅ Database connected successfully")
	return db
}

func GetDB() *gorm.DB {
	if DB == nil {
		return InitDB()
	}
	return DB
}
