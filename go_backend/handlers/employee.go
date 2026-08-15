package handlers

import (
	"math"
	"net/http"
	"time"

	"go_backend/models"
	"go_backend/utils"

	"github.com/gin-gonic/gin"
)

// EmployeeSelectPage 员工分页查询
func EmployeeSelectPage(c *gin.Context) {
	var req models.EmployeePageRequest
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
	query := db.Model(&models.Employee{}).Where("is_deleted = ?", 0)

	if req.Name != "" {
		query = query.Where("name LIKE ?", "%"+req.Name+"%")
	}
	if req.Username != "" {
		query = query.Where("username LIKE ?", "%"+req.Username+"%")
	}

	var total int64
	query.Count(&total)

	var records []models.Employee
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

// EmployeeInsert 员工新增
func EmployeeInsert(c *gin.Context) {
	var emp models.Employee
	if err := c.ShouldBindJSON(&emp); err != nil {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "请求参数错误",
		})
		return
	}

	emp.ID = generateID()
	emp.IsDeleted = 0
	now := time.Now()
	emp.CreateTime = &now
	emp.UpdateTime = &now

	db := utils.GetDB()
	if err := db.Create(&emp).Error; err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "数据库错误: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, emp)
}

// EmployeeUpdate 员工更新
func EmployeeUpdate(c *gin.Context) {
	var emp models.Employee
	if err := c.ShouldBindJSON(&emp); err != nil {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "请求参数错误",
		})
		return
	}

	if emp.ID == "" {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "缺少员工 ID",
		})
		return
	}

	now := time.Now()
	emp.UpdateTime = &now

	db := utils.GetDB()
	if err := db.Model(&models.Employee{}).Where("id = ?", emp.ID).Updates(&emp).Error; err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "数据库错误: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, emp)
}

// EmployeeDelete 员工删除
func EmployeeDelete(c *gin.Context) {
	id := c.Param("id")
	if id == "" {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "缺少员工 ID",
		})
		return
	}

	db := utils.GetDB()
	if err := db.Model(&models.Employee{}).Where("id = ?", id).Update("is_deleted", 1).Error; err != nil {
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
