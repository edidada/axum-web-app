package handlers

import (
	"math/rand"
	"strconv"
)

// generateID 生成随机 ID
func generateID() string {
	return strconv.FormatUint(rand.Uint64(), 10)
}
