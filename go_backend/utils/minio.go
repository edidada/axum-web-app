package utils

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"log"
	"net/http"
	"path/filepath"
	"strings"
	"time"

	"go_backend/config"

	"github.com/google/uuid"
	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
)

var MinioClient *minio.Client

func InitMinio() *minio.Client {
	cfg := config.GetConfig()

	client, err := minio.New(cfg.Minio.Endpoint, &minio.Options{
		Creds:  credentials.NewStaticV4(cfg.Minio.AccessKey, cfg.Minio.SecretKey, ""),
		Secure: cfg.Minio.UseSSL,
	})

	if err != nil {
		log.Printf("⚠️ Failed to create MinIO client: %v", err)
		return nil
	}

	// 检查 bucket 是否存在，不存在则创建
	ctx := context.Background()
	exists, err := client.BucketExists(ctx, cfg.Minio.Bucket)
	if err != nil {
		log.Printf("⚠️ Failed to check bucket existence: %v", err)
		log.Println("⚠️ MinIO not available, image upload features will be disabled")
		return nil
	}

	if !exists {
		err = client.MakeBucket(ctx, cfg.Minio.Bucket, minio.MakeBucketOptions{})
		if err != nil {
			log.Printf("⚠️ Failed to create bucket: %v", err)
			return nil
		}
		fmt.Printf("✅ Created bucket: %s\n", cfg.Minio.Bucket)
	}

	MinioClient = client
	fmt.Println("✅ MinIO connected successfully")
	return client
}

func GetMinio() *minio.Client {
	if MinioClient == nil {
		return InitMinio()
	}
	return MinioClient
}

// UploadFile 上传文件到 MinIO
func UploadFile(objectName string, data []byte, contentType string) (string, error) {
	cfg := config.GetConfig()
	client := GetMinio()
	ctx := context.Background()

	reader := bytes.NewReader(data)

	_, err := client.PutObject(ctx, cfg.Minio.Bucket, objectName, reader, int64(len(data)),
		minio.PutObjectOptions{ContentType: contentType})

	if err != nil {
		return "", fmt.Errorf("failed to upload file: %w", err)
	}

	// 生成访问 URL
	protocol := "http"
	if cfg.Minio.UseSSL {
		protocol = "https"
	}
	url := fmt.Sprintf("%s://%s/%s/%s", protocol, cfg.Minio.Endpoint, cfg.Minio.Bucket, objectName)

	return url, nil
}

// DownloadFile 从 MinIO 下载文件
func DownloadFile(objectName string) ([]byte, error) {
	cfg := config.GetConfig()
	client := GetMinio()
	ctx := context.Background()

	object, err := client.GetObject(ctx, cfg.Minio.Bucket, objectName, minio.GetObjectOptions{})
	if err != nil {
		return nil, fmt.Errorf("failed to get object: %w", err)
	}
	defer object.Close()

	data, err := io.ReadAll(object)
	if err != nil {
		return nil, fmt.Errorf("failed to read object: %w", err)
	}

	return data, nil
}

// DeleteFile 从 MinIO 删除文件
func DeleteFile(objectName string) error {
	cfg := config.GetConfig()
	client := GetMinio()
	ctx := context.Background()

	err := client.RemoveObject(ctx, cfg.Minio.Bucket, objectName, minio.RemoveObjectOptions{})
	if err != nil {
		return fmt.Errorf("failed to delete object: %w", err)
	}

	return nil
}

// GetFileURL 获取文件访问 URL
func GetFileURL(objectName string) string {
	cfg := config.GetConfig()

	protocol := "http"
	if cfg.Minio.UseSSL {
		protocol = "https"
	}

	return fmt.Sprintf("%s://%s/%s/%s", protocol, cfg.Minio.Endpoint, cfg.Minio.Bucket, objectName)
}

// DownloadAndUploadImage 从 URL 下载图片并上传到 MinIO
func DownloadAndUploadImage(imageURL string, objectName string) (string, error) {
	// 下载图片
	resp, err := http.Get(imageURL)
	if err != nil {
		return "", fmt.Errorf("failed to download image: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return "", fmt.Errorf("failed to download image, status: %d", resp.StatusCode)
	}

	// 读取图片数据
	data, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("failed to read image data: %w", err)
	}

	// 获取 content type
	contentType := resp.Header.Get("Content-Type")
	if contentType == "" {
		contentType = "image/jpeg"
	}

	// 如果 objectName 为空，生成一个
	if objectName == "" {
		ext := filepath.Ext(imageURL)
		if ext == "" {
			ext = ".jpg"
		}
		// 移除查询参数
		ext = strings.Split(ext, "?")[0]
		objectName = fmt.Sprintf("images/%s%s", uuid.New().String(), ext)
	}

	// 上传到 MinIO
	url, err := UploadFile(objectName, data, contentType)
	if err != nil {
		return "", err
	}

	return url, nil
}

// GeneratePresignedURL 生成预签名 URL
func GeneratePresignedURL(objectName string, expiry time.Duration) (string, error) {
	cfg := config.GetConfig()
	client := GetMinio()
	ctx := context.Background()

	url, err := client.PresignedGetObject(ctx, cfg.Minio.Bucket, objectName, expiry, nil)
	if err != nil {
		return "", fmt.Errorf("failed to generate presigned URL: %w", err)
	}

	return url.String(), nil
}
