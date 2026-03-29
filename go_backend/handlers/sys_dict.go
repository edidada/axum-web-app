package handlers

import (
	"math"
	"net/http"
	"time"

	"go_backend/models"
	"go_backend/utils"

	"github.com/gin-gonic/gin"
)

// SysDictSelectPage 字典分页查询
func SysDictSelectPage(c *gin.Context) {
	var req models.SysDictPageRequest
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
	query := db.Model(&models.SysDict{})

	if req.Code != "" {
		query = query.Where("code LIKE ?", "%"+req.Code+"%")
	}
	if req.Name != "" {
		query = query.Where("name LIKE ?", "%"+req.Name+"%")
	}

	var total int64
	query.Count(&total)

	var records []models.SysDict
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

// SysDictInsert 字典新增
func SysDictInsert(c *gin.Context) {
	var dict models.SysDict
	if err := c.ShouldBindJSON(&dict); err != nil {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "请求参数错误",
		})
		return
	}

	dict.ID = generateID()
	now := time.Now()
	dict.CreateDate = &now

	db := utils.GetDB()
	if err := db.Create(&dict).Error; err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "数据库错误: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, dict)
}

// SysDictUpdate 字典更新
func SysDictUpdate(c *gin.Context) {
	var dict models.SysDict
	if err := c.ShouldBindJSON(&dict); err != nil {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "请求参数错误",
		})
		return
	}

	if dict.ID == "" {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "缺少字典 ID",
		})
		return
	}

	db := utils.GetDB()
	if err := db.Model(&models.SysDict{}).Where("id = ?", dict.ID).Updates(&dict).Error; err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "数据库错误: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, dict)
}

// SysDictDelete 字典删除
func SysDictDelete(c *gin.Context) {
	id := c.Param("id")
	if id == "" {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "缺少字典 ID",
		})
		return
	}

	db := utils.GetDB()
	if err := db.Where("id = ?", id).Delete(&models.SysDict{}).Error; err != nil {
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
