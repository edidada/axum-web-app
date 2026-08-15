package handlers

import (
	"math"
	"net/http"
	"time"

	"go_backend/models"
	"go_backend/utils"

	"github.com/gin-gonic/gin"
)

// CategorySelectPage 分类分页查询
func CategorySelectPage(c *gin.Context) {
	var req models.CategoryPageRequest
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
	query := db.Model(&models.Category{}).Where("is_deleted = ?", 0)

	if req.Name != "" {
		query = query.Where("name LIKE ?", "%"+req.Name+"%")
	}
	if req.Type > 0 {
		query = query.Where("type = ?", req.Type)
	}

	var total int64
	query.Count(&total)

	var records []models.Category
	offset := (req.Page - 1) * req.Size
	query.Order("sort ASC").Limit(req.Size).Offset(offset).Find(&records)

	c.JSON(http.StatusOK, models.PageResult{
		Records: records,
		Total:   total,
		Size:    req.Size,
		Current: req.Page,
		Pages:   int(math.Ceil(float64(total) / float64(req.Size))),
	})
}

// CategoryInsert 分类新增
func CategoryInsert(c *gin.Context) {
	var cat models.Category
	if err := c.ShouldBindJSON(&cat); err != nil {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "请求参数错误",
		})
		return
	}

	cat.ID = generateID()
	cat.IsDeleted = 0
	now := time.Now()
	cat.CreateTime = &now
	cat.UpdateTime = &now

	db := utils.GetDB()
	if err := db.Create(&cat).Error; err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "数据库错误: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, cat)
}

// CategoryUpdate 分类更新
func CategoryUpdate(c *gin.Context) {
	var cat models.Category
	if err := c.ShouldBindJSON(&cat); err != nil {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "请求参数错误",
		})
		return
	}

	if cat.ID == "" {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "缺少分类 ID",
		})
		return
	}

	now := time.Now()
	cat.UpdateTime = &now

	db := utils.GetDB()
	if err := db.Model(&models.Category{}).Where("id = ?", cat.ID).Updates(&cat).Error; err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "数据库错误: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, cat)
}

// CategoryDelete 分类删除
func CategoryDelete(c *gin.Context) {
	id := c.Param("id")
	if id == "" {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "缺少分类 ID",
		})
		return
	}

	db := utils.GetDB()
	if err := db.Model(&models.Category{}).Where("id = ?", id).Update("is_deleted", 1).Error; err != nil {
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
