package utils

import (
	"context"
	"fmt"
	"log"
	"time"

	"go_backend/config"

	"github.com/redis/go-redis/v9"
)

var RedisClient *redis.Client
var ctx = context.Background()

func InitRedis() *redis.Client {
	cfg := config.GetConfig()
	
	RedisClient = redis.NewClient(&redis.Options{
		Addr:     fmt.Sprintf("%s:%d", cfg.Redis.IP, cfg.Redis.Port),
		Password: "", // no password set
		DB:       0,  // use default DB
	})
	
	// 测试连接
	_, err := RedisClient.Ping(ctx).Result()
	if err != nil {
		log.Fatalf("Failed to connect to Redis: %v", err)
	}
	
	fmt.Println("✅ Redis connected successfully")
	return RedisClient
}

func GetRedis() *redis.Client {
	if RedisClient == nil {
		return InitRedis()
	}
	return RedisClient
}

// SetSession 设置会话
func SetSession(sessionID string, username string, expiration time.Duration) error {
	cfg := config.GetConfig()
	key := cfg.Redis.KeyPrefix + sessionID
	return RedisClient.Set(ctx, key, username, expiration).Err()
}

// GetSession 获取会话
func GetSession(sessionID string) (string, error) {
	cfg := config.GetConfig()
	key := cfg.Redis.KeyPrefix + sessionID
	return RedisClient.Get(ctx, key).Result()
}

// DeleteSession 删除会话
func DeleteSession(sessionID string) error {
	cfg := config.GetConfig()
	key := cfg.Redis.KeyPrefix + sessionID
	return RedisClient.Del(ctx, key).Err()
}

// ExtendSession 延长会话过期时间
func ExtendSession(sessionID string, expiration time.Duration) error {
	cfg := config.GetConfig()
	key := cfg.Redis.KeyPrefix + sessionID
	return RedisClient.Expire(ctx, key, expiration).Err()
}
