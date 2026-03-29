package handlers

import (
	"math"
	"net/http"

	"go_backend/models"
	"go_backend/utils"

	"github.com/gin-gonic/gin"
)

// OrdersSelectPage 订单分页查询
func OrdersSelectPage(c *gin.Context) {
	var req models.OrdersPageRequest
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
	query := db.Model(&models.Orders{})

	if req.Number != "" {
		query = query.Where("number LIKE ?", "%"+req.Number+"%")
	}
	if req.Status > 0 {
		query = query.Where("status = ?", req.Status)
	}
	if req.Phone != "" {
		query = query.Where("phone LIKE ?", "%"+req.Phone+"%")
	}
	if req.Consignee != "" {
		query = query.Where("consignee LIKE ?", "%"+req.Consignee+"%")
	}

	var total int64
	query.Count(&total)

	var records []models.Orders
	offset := (req.Page - 1) * req.Size
	query.Order("order_time DESC").Limit(req.Size).Offset(offset).Find(&records)

	c.JSON(http.StatusOK, models.PageResult{
		Records: records,
		Total:   total,
		Size:    req.Size,
		Current: req.Page,
		Pages:   int(math.Ceil(float64(total) / float64(req.Size))),
	})
}
