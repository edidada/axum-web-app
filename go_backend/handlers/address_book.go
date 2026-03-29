package handlers

import (
	"math"
	"net/http"
	"time"

	"go_backend/models"
	"go_backend/utils"

	"github.com/gin-gonic/gin"
)

// AddressBookSelectPage 地址簿分页查询
func AddressBookSelectPage(c *gin.Context) {
	var req models.AddressBookPageRequest
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
	query := db.Model(&models.AddressBook{}).Where("is_deleted = ?", 0)

	if req.Consignee != "" {
		query = query.Where("consignee LIKE ?", "%"+req.Consignee+"%")
	}
	if req.Phone != "" {
		query = query.Where("phone LIKE ?", "%"+req.Phone+"%")
	}

	var total int64
	query.Count(&total)

	var records []models.AddressBook
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

// AddressBookInsert 地址簿新增
func AddressBookInsert(c *gin.Context) {
	var addr models.AddressBook
	if err := c.ShouldBindJSON(&addr); err != nil {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "请求参数错误",
		})
		return
	}

	addr.ID = generateID()
	addr.IsDeleted = 0
	now := time.Now()
	addr.CreateTime = &now
	addr.UpdateTime = &now

	db := utils.GetDB()
	if err := db.Create(&addr).Error; err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "数据库错误: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, addr)
}

// AddressBookUpdate 地址簿更新
func AddressBookUpdate(c *gin.Context) {
	var addr models.AddressBook
	if err := c.ShouldBindJSON(&addr); err != nil {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "请求参数错误",
		})
		return
	}

	if addr.ID == "" {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "缺少地址 ID",
		})
		return
	}

	now := time.Now()
	addr.UpdateTime = &now

	db := utils.GetDB()
	if err := db.Model(&models.AddressBook{}).Where("id = ?", addr.ID).Updates(&addr).Error; err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "数据库错误: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, addr)
}

// AddressBookDelete 地址簿删除
func AddressBookDelete(c *gin.Context) {
	id := c.Param("id")
	if id == "" {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "缺少地址 ID",
		})
		return
	}

	db := utils.GetDB()
	// 软删除
	if err := db.Model(&models.AddressBook{}).Where("id = ?", id).Update("is_deleted", 1).Error; err != nil {
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
