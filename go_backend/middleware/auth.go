package middleware

import (
	"net/http"
	"strings"
	"time"

	"go_backend/config"
	"go_backend/models"
	"go_backend/utils"

	"github.com/gin-gonic/gin"
)

// AuthMiddleware 认证中间件
func AuthMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		// 从 Cookie 中获取 session
		sessionID, err := c.Cookie("session")
		if err != nil {
			// 尝试从 Header 获取
			authHeader := c.GetHeader("Authorization")
			if authHeader != "" {
				parts := strings.Split(authHeader, " ")
				if len(parts) == 2 && parts[0] == "Bearer" {
					sessionID = parts[1]
				}
			}
		}

		if sessionID == "" {
			c.JSON(http.StatusUnauthorized, models.Response{
				Code:    1,
				Status:  "error",
				Message: "未登录",
			})
			c.Abort()
			return
		}

		// 验证 session
		username, err := utils.GetSession(sessionID)
		if err != nil {
			c.JSON(http.StatusUnauthorized, models.Response{
				Code:    1,
				Status:  "error",
				Message: "会话已过期",
			})
			c.Abort()
			return
		}

		// 延长 session 过期时间
		cfg := config.GetConfig()
		utils.ExtendSession(sessionID, time.Duration(cfg.Redis.Expiration)*time.Second)

		// 将用户信息存入上下文
		c.Set("username", username)
		c.Set("session_id", sessionID)

		c.Next()
	}
}

// CORSMiddleware 跨域中间件
func CORSMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Writer.Header().Set("Access-Control-Allow-Origin", c.Request.Header.Get("Origin"))
		c.Writer.Header().Set("Access-Control-Allow-Credentials", "true")
		c.Writer.Header().Set("Access-Control-Allow-Headers", "Content-Type, Content-Length, Accept-Encoding, X-CSRF-Token, Authorization, accept, origin, Cache-Control, X-Requested-With")
		c.Writer.Header().Set("Access-Control-Allow-Methods", "POST, OPTIONS, GET, PUT, DELETE")

		if c.Request.Method == "OPTIONS" {
			c.AbortWithStatus(204)
			return
		}

		c.Next()
	}
}

// LoggerMiddleware 日志中间件
func LoggerMiddleware() gin.HandlerFunc {
	return gin.LoggerWithFormatter(func(param gin.LogFormatterParams) string {
		return ""
	})
}
