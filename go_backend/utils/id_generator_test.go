package utils

import (
	"fmt"
	"strings"
	"testing"
)

func TestIDGenerator(t *testing.T) {
	// 测试1: 默认生成器
	fmt.Println("=== 默认生成器 (无前缀, 长度20) ===")
	gen1 := NewIDGenerator("", 20)
	for i := 0; i < 5; i++ {
		id := gen1.Generate()
		fmt.Printf("ID %d: %s (长度: %d)\n", i+1, id, len(id))
	}

	// 测试2: 带前缀的生成器
	fmt.Println("\n=== 用户ID生成器 (前缀: USER, 长度24) ===")
	gen2 := NewIDGenerator("USER", 24)
	for i := 0; i < 5; i++ {
		id := gen2.Generate()
		fmt.Printf("ID %d: %s (长度: %d)\n", i+1, id, len(id))
		if !strings.HasPrefix(id, "USER-") {
			t.Errorf("ID应该以 USER- 开头: %s", id)
		}
	}

	// 测试3: 订单ID生成器
	fmt.Println("\n=== 订单ID生成器 (前缀: ORDER, 长度32) ===")
	gen3 := NewIDGenerator("ORDER", 32)
	for i := 0; i < 5; i++ {
		id := gen3.Generate()
		fmt.Printf("ID %d: %s (长度: %d)\n", i+1, id, len(id))
	}

	// 测试4: 不同前缀生成
	fmt.Println("\n=== 使用同一生成器生成不同前缀的ID ===")
	gen4 := NewIDGenerator("", 24)
	fmt.Printf("员工ID: %s\n", gen4.GenerateWithPrefix("EMP"))
	fmt.Printf("产品ID: %s\n", gen4.GenerateWithPrefix("PROD"))
	fmt.Printf("分类ID: %s\n", gen4.GenerateWithPrefix("CAT"))

	// 测试5: 长ID
	fmt.Println("\n=== 长ID生成器 (前缀: LONG, 长度48) ===")
	gen5 := NewIDGenerator("LONG", 48)
	for i := 0; i < 3; i++ {
		id := gen5.Generate()
		fmt.Printf("ID %d: %s (长度: %d)\n", i+1, id, len(id))
	}

	// 测试6: 短ID
	fmt.Println("\n=== 短ID生成器 (前缀: S, 长度12) ===")
	gen6 := NewIDGenerator("S", 12)
	for i := 0; i < 3; i++ {
		id := gen6.Generate()
		fmt.Printf("ID %d: %s (长度: %d)\n", i+1, id, len(id))
	}

	// 测试7: 并发测试
	fmt.Println("\n=== 并发生成测试 ===")
	gen7 := NewIDGenerator("CONC", 24)
	idChan := make(chan string, 100)
	for i := 0; i < 10; i++ {
		go func() {
			for j := 0; j < 10; j++ {
				idChan <- gen7.Generate()
			}
		}()
	}

	idMap := make(map[string]bool)
	for i := 0; i < 100; i++ {
		id := <-idChan
		if idMap[id] {
			t.Errorf("发现重复ID: %s", id)
		}
		idMap[id] = true
	}
	fmt.Printf("生成了 %d 个唯一ID\n", len(idMap))

	// 测试8: 获取生成器信息
	fmt.Println("\n=== 生成器信息 ===")
	gen8 := NewIDGenerator("INFO", 30)
	info := gen8.GetInfo()
	fmt.Printf("前缀: %v\n", info["prefix"])
	fmt.Printf("总长度: %v\n", info["totalLength"])
	fmt.Printf("时间戳长度: %v\n", info["timeLength"])
	fmt.Printf("随机数长度: %v\n", info["randomLength"])
	fmt.Printf("序列号长度: %v\n", info["seqLength"])
}

func TestGlobalFunctions(t *testing.T) {
	fmt.Println("\n=== 全局便捷函数测试 ===")

	// 使用默认生成器
	id1 := GenerateID()
	fmt.Printf("默认ID: %s\n", id1)

	// 使用前缀
	id2 := GenerateIDWithPrefix("TEST")
	fmt.Printf("带前缀ID: %s\n", id2)

	// 使用完整配置
	id3 := GenerateIDWithConfig("CONFIG", 28)
	fmt.Printf("配置ID: %s\n", id3)
}
