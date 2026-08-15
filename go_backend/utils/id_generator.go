package utils

import (
	"fmt"
	"math/rand"
	"sync"
	"time"
)

// IDGenerator 分布式ID生成器
// 支持自定义前缀和总长度
// 格式: 前缀 + 时间戳(16进制) + 随机数 + 序列号
// 示例: USER-1234567890ABCDEF-1234-001

type IDGenerator struct {
	mu        sync.Mutex
	prefix    string // ID前缀
	totalLen  int    // 总长度
	sequence  int    // 序列号
	lastTime  int64  // 上次生成时间
	randomLen int    // 随机数长度
	seqLen    int    // 序列号长度
	timeLen   int    // 时间戳长度
}

var (
	// 默认生成器实例
	defaultGenerator *IDGenerator
	once             sync.Once
)

// GetDefaultGenerator 获取默认ID生成器
func GetDefaultGenerator() *IDGenerator {
	once.Do(func() {
		defaultGenerator = NewIDGenerator("", 20)
	})
	return defaultGenerator
}

// NewIDGenerator 创建新的ID生成器
// prefix: ID前缀，如 "USER", "ORDER" 等
// totalLen: ID总长度（包含前缀、分隔符等）
func NewIDGenerator(prefix string, totalLen int) *IDGenerator {
	if totalLen < 10 {
		totalLen = 20 // 最小长度
	}
	if totalLen > 64 {
		totalLen = 64 // 最大长度
	}

	g := &IDGenerator{
		prefix:   prefix,
		totalLen: totalLen,
		sequence: 0,
	}

	// 计算各部分长度
	g.calculateLengths()

	return g
}

// calculateLengths 计算各部分长度
func (g *IDGenerator) calculateLengths() {
	prefixLen := len(g.prefix)
	separatorLen := 0
	if prefixLen > 0 {
		separatorLen = 1 // 前缀后的分隔符 "-"
	}

	// 可用长度 = 总长度 - 前缀长度 - 分隔符长度
	availableLen := g.totalLen - prefixLen - separatorLen

	// 序列号长度（至少3位）
	g.seqLen = 3
	if availableLen > 20 {
		g.seqLen = 4
	}

	// 随机数长度（至少4位）
	g.randomLen = 4
	if availableLen > 25 {
		g.randomLen = 6
	}

	// 时间戳长度 = 剩余长度
	g.timeLen = availableLen - g.randomLen - g.seqLen
	if g.timeLen < 8 {
		g.timeLen = 8 // 最小8位时间戳
		// 如果不够，减少随机数长度
		if availableLen < g.timeLen+g.randomLen+g.seqLen {
			g.randomLen = availableLen - g.timeLen - g.seqLen
			if g.randomLen < 2 {
				g.randomLen = 2
			}
		}
	}
}

// Generate 生成ID
func (g *IDGenerator) Generate() string {
	g.mu.Lock()
	defer g.mu.Unlock()

	now := time.Now().UnixMilli()

	// 如果是同一毫秒，递增序列号
	if now == g.lastTime {
		g.sequence++
	} else {
		g.sequence = 0
		g.lastTime = now
	}

	// 构建ID各部分
	var id string

	// 1. 前缀
	if g.prefix != "" {
		id = g.prefix + "-"
	}

	// 2. 时间戳（16进制，大写）
	timeHex := fmt.Sprintf("%0*X", g.timeLen, now)
	if len(timeHex) > g.timeLen {
		timeHex = timeHex[len(timeHex)-g.timeLen:]
	}
	id += timeHex

	// 3. 随机数
	randomNum := rand.Intn(intPow(10, g.randomLen))
	randomStr := fmt.Sprintf("%0*d", g.randomLen, randomNum)
	id += "-" + randomStr

	// 4. 序列号
	seqNum := g.sequence % intPow(10, g.seqLen)
	seqStr := fmt.Sprintf("%0*d", g.seqLen, seqNum)
	id += "-" + seqStr

	return id
}

// GenerateWithPrefix 使用指定前缀生成ID
func (g *IDGenerator) GenerateWithPrefix(prefix string) string {
	g.mu.Lock()
	defer g.mu.Unlock()

	now := time.Now().UnixMilli()

	// 如果是同一毫秒，递增序列号
	if now == g.lastTime {
		g.sequence++
	} else {
		g.sequence = 0
		g.lastTime = now
	}

	// 构建ID
	var id string

	// 1. 前缀
	if prefix != "" {
		id = prefix + "-"
	}

	// 计算各部分长度（基于原始配置）
	prefixLen := len(prefix)
	separatorLen := 0
	if prefixLen > 0 {
		separatorLen = 1
	}
	availableLen := g.totalLen - prefixLen - separatorLen

	seqLen := g.seqLen
	randomLen := g.randomLen
	timeLen := availableLen - randomLen - seqLen
	if timeLen < 8 {
		timeLen = 8
	}

	// 2. 时间戳
	timeHex := fmt.Sprintf("%0*X", timeLen, now)
	if len(timeHex) > timeLen {
		timeHex = timeHex[len(timeHex)-timeLen:]
	}
	id += timeHex

	// 3. 随机数
	randomNum := rand.Intn(intPow(10, randomLen))
	randomStr := fmt.Sprintf("%0*d", randomLen, randomNum)
	id += "-" + randomStr

	// 4. 序列号
	seqNum := g.sequence % intPow(10, seqLen)
	seqStr := fmt.Sprintf("%0*d", seqLen, seqNum)
	id += "-" + seqStr

	return id
}

// intPow 计算10的n次方
func intPow(n, m int) int {
	if m == 0 {
		return 1
	}
	result := n
	for i := 2; i <= m; i++ {
		result *= n
	}
	return result
}

// SetPrefix 设置前缀
func (g *IDGenerator) SetPrefix(prefix string) {
	g.mu.Lock()
	defer g.mu.Unlock()
	g.prefix = prefix
	g.calculateLengths()
}

// SetTotalLength 设置总长度
func (g *IDGenerator) SetTotalLength(length int) {
	g.mu.Lock()
	defer g.mu.Unlock()
	if length >= 10 && length <= 64 {
		g.totalLen = length
		g.calculateLengths()
	}
}

// GetInfo 获取生成器信息
func (g *IDGenerator) GetInfo() map[string]interface{} {
	g.mu.Lock()
	defer g.mu.Unlock()

	return map[string]interface{}{
		"prefix":       g.prefix,
		"totalLength":  g.totalLen,
		"timeLength":   g.timeLen,
		"randomLength": g.randomLen,
		"seqLength":    g.seqLen,
	}
}

// 全局便捷函数

// GenerateID 使用默认生成器生成ID
func GenerateID() string {
	return GetDefaultGenerator().Generate()
}

// GenerateIDWithPrefix 使用指定前缀生成ID
func GenerateIDWithPrefix(prefix string) string {
	return GetDefaultGenerator().GenerateWithPrefix(prefix)
}

// GenerateIDWithConfig 使用完整配置生成ID
func GenerateIDWithConfig(prefix string, totalLen int) string {
	gen := NewIDGenerator(prefix, totalLen)
	return gen.Generate()
}
