package handlers

import (
	"math"
	"net/http"

	"go_backend/models"
	"go_backend/utils"

	"github.com/gin-gonic/gin"
)

// UserSelectPage 用户分页查询
func UserSelectPage(c *gin.Context) {
	var req models.UserPageRequest
	if err := c.ShouldBindQuery(&req); err != nil {
		c.ShouldBindJSON(&req)
	}

	if req.Page <= 0 {
		req.Page = 1
	}
	if req.Size <= 0 {
		req.Size = 10
	}

	db := utils.GetDB()
	query := db.Model(&models.User{}).Where("is_deleted = ?", 0)

	if req.Name != "" {
		query = query.Where("name LIKE ?", "%"+req.Name+"%")
	}
	if req.Phone != "" {
		query = query.Where("phone LIKE ?", "%"+req.Phone+"%")
	}

	var total int64
	query.Count(&total)

	var records []models.User
	offset := (req.Page - 1) * req.Size
	query.Order("create_time DESC").Limit(req.Size).Offset(offset).Find(&records)

	c.JSON(http.StatusOK, models.PageResult{
		Records: records,
		Total:   total,
		Size:    req.Size,
		Current: req.Page,
		Pages:   int(math.Ceil(float64(total) / float64(req.Size))),
	})
}
