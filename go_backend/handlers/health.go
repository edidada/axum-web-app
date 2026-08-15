package handlers

import (
	"net/http"
	"runtime"
	"time"

	"go_backend/models"

	"github.com/gin-gonic/gin"
)

// HealthCheck 健康检查
func HealthCheck(c *gin.Context) {
	c.JSON(http.StatusOK, models.Response{
		Code:   0,
		Status: "success",
		Data:   "OK",
	})
}

// HealthCheckDetailed 详细健康检查
func HealthCheckDetailed(c *gin.Context) {
	var m runtime.MemStats
	runtime.ReadMemStats(&m)

	c.JSON(http.StatusOK, models.Response{
		Code:   0,
		Status: "success",
		Data: gin.H{
			"status":    "UP",
			"timestamp": time.Now().Format(time.RFC3339),
			"memory": gin.H{
				"alloc":      m.Alloc,
				"total_alloc": m.TotalAlloc,
				"sys":        m.Sys,
				"num_gc":     m.NumGC,
			},
			"goroutines": runtime.NumGoroutine(),
		},
	})
}

// ReadinessCheck 就绪检查
func ReadinessCheck(c *gin.Context) {
	c.JSON(http.StatusOK, models.Response{
		Code:   0,
		Status: "success",
		Data:   "READY",
	})
}

// LivenessCheck 存活检查
func LivenessCheck(c *gin.Context) {
	c.JSON(http.StatusOK, models.Response{
		Code:   0,
		Status: "success",
		Data:   "ALIVE",
	})
}
