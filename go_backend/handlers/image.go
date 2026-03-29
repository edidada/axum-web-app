package handlers

import (
	"net/http"
	"path/filepath"
	"strings"

	"go_backend/models"
	"go_backend/utils"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

// UploadImageFromURL 从 URL 上传图片到 MinIO
func UploadImageFromURL(c *gin.Context) {
	var req models.UploadImageRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "请求参数错误: " + err.Error(),
		})
		return
	}

	// 如果 object_name 为空，生成一个
	objectName := req.ObjectName
	if objectName == "" {
		ext := filepath.Ext(req.ImageURL)
		if ext == "" {
			ext = ".jpg"
		}
		// 移除查询参数
		ext = strings.Split(ext, "?")[0]
		objectName = "images/" + uuid.New().String() + ext
	}

	// 下载并上传图片
	url, err := utils.DownloadAndUploadImage(req.ImageURL, objectName)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "上传失败: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, models.Response{
		Code:   0,
		Status: "success",
		Data: gin.H{
			"url":         url,
			"object_name": objectName,
		},
	})
}

// DeleteImage 删除 MinIO 中的图片
func DeleteImage(c *gin.Context) {
	objectName := c.Param("objectName")
	if objectName == "" {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "缺少对象名称",
		})
		return
	}

	err := utils.DeleteFile(objectName)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.Response{
			Code:    1,
			Status:  "error",
			Message: "删除失败: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, models.Response{
		Code:   0,
		Status: "success",
		Data:   "删除成功",
	})
}

// GetImageURL 获取图片访问 URL
func GetImageURL(c *gin.Context) {
	objectName := c.Param("objectName")
	if objectName == "" {
		c.JSON(http.StatusBadRequest, models.Response{
			Code:    1,
			Status:  "error",
			Message: "缺少对象名称",
		})
		return
	}

	url := utils.GetFileURL(objectName)

	c.JSON(http.StatusOK, models.Response{
		Code:   0,
		Status: "success",
		Data: gin.H{
			"url": url,
		},
	})
}
