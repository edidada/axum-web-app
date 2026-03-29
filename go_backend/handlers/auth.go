package handlers

import (
	"net/http"
	"time"

	"go_backend/config"
	"go_backend/models"
	"go_backend/utils"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

// Login 登录
func Login(c *gin.Context) {
	var req models.LoginRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "请求参数错误",
		})
		return
	}

	cfg := config.GetConfig()

	// 验证用户名和密码
	if req.Username != cfg.Auth.Username || req.Password != cfg.Auth.Password {
		c.JSON(http.StatusUnauthorized, models.Response{
			Code:    1,
			Status:  "error",
			Message: "用户名或密码错误",
		})
		return
	}

	// 生成 session ID
	sessionID := uuid.New().String()

	// 存储到 Redis
	expiration := time.Duration(cfg.Redis.Expiration) * time.Second
	err := utils.SetSession(sessionID, req.Username, expiration)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "服务器错误",
		})
		return
	}

	// 设置 Cookie
	c.SetCookie("session", sessionID, cfg.Redis.Expiration, "/", "", false, true)

	c.JSON(http.StatusOK, models.LoginResponse{
		Code:      0,
		Status:    "success",
		SessionID: sessionID,
	})
}

// Logout 登出
func Logout(c *gin.Context) {
	sessionID, _ := c.Get("session_id")
	if sessionID != nil {
		utils.DeleteSession(sessionID.(string))
	}

	// 清除 Cookie
	c.SetCookie("session", "", -1, "/", "", false, true)

	c.JSON(http.StatusOK, models.Response{
		Code:   0,
		Status: "success",
		Data:   "已登出",
	})
}
