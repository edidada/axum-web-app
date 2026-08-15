package handlers

import (
	"math"
	"net/http"

	"go_backend/models"
	"go_backend/utils"

	"github.com/gin-gonic/gin"
)

// SetmealSelectPage 套餐分页查询
func SetmealSelectPage(c *gin.Context) {
	var req models.SetmealPageRequest
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
	query := db.Model(&models.Setmeal{}).Where("is_deleted = ?", 0)

	if req.Name != "" {
		query = query.Where("name LIKE ?", "%"+req.Name+"%")
	}
	if req.CategoryID != "" {
		query = query.Where("category_id = ?", req.CategoryID)
	}
	if req.Status > 0 {
		query = query.Where("status = ?", req.Status)
	}

	var total int64
	query.Count(&total)

	var records []models.Setmeal
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
