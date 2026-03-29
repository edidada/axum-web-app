package handlers

import (
	"go_backend/utils"
)

// generateID 生成随机 ID（使用分布式ID生成器）
func generateID() string {
	return utils.GenerateID()
}

// generateIDWithPrefix 使用指定前缀生成ID
func generateIDWithPrefix(prefix string) string {
	return utils.GenerateIDWithPrefix(prefix)
}
