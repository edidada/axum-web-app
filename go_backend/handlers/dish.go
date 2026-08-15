package handlers

import (
	"math"
	"net/http"
	"time"

	"go_backend/models"
	"go_backend/utils"

	"github.com/gin-gonic/gin"
)

// DishSelectPage 菜品分页查询
func DishSelectPage(c *gin.Context) {
	var req models.DishPageRequest
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
	query := db.Model(&models.Dish{}).Where("is_deleted = ?", 0)

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

	var records []models.Dish
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

// DishInsert 菜品新增
func DishInsert(c *gin.Context) {
	var dish models.Dish
	if err := c.ShouldBindJSON(&dish); err != nil {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "请求参数错误",
		})
		return
	}

	dish.ID = generateID()
	dish.IsDeleted = 0
	now := time.Now()
	dish.CreateTime = &now
	dish.UpdateTime = &now

	db := utils.GetDB()
	if err := db.Create(&dish).Error; err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "数据库错误: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, dish)
}

// DishUpdate 菜品更新
func DishUpdate(c *gin.Context) {
	var dish models.Dish
	if err := c.ShouldBindJSON(&dish); err != nil {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "请求参数错误",
		})
		return
	}

	if dish.ID == "" {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "缺少菜品 ID",
		})
		return
	}

	now := time.Now()
	dish.UpdateTime = &now

	db := utils.GetDB()
	if err := db.Model(&models.Dish{}).Where("id = ?", dish.ID).Updates(&dish).Error; err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "数据库错误: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, dish)
}

// DishDelete 菜品删除
func DishDelete(c *gin.Context) {
	id := c.Param("id")
	if id == "" {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "缺少菜品 ID",
		})
		return
	}

	db := utils.GetDB()
	if err := db.Model(&models.Dish{}).Where("id = ?", id).Update("is_deleted", 1).Error; err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "数据库错误: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, models.Response{
		Code:   0,
		Status: "success",
		Data:   "删除成功",
	})
}
