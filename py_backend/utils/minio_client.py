from minio import Minio
from minio.error import S3Error
from config.settings import settings
import io

# MinIO 客户端
minio_client = None

try:
    minio_client = Minio(
        settings.minio_endpoint,
        access_key=settings.minio_access_key,
        secret_key=settings.minio_secret_key,
        secure=settings.minio_use_ssl
    )
    
    # 检查 bucket 是否存在
    if not minio_client.bucket_exists(settings.minio_bucket):
        minio_client.make_bucket(settings.minio_bucket)
        print(f"✅ Created bucket: {settings.minio_bucket}")
    else:
        print(f"✅ Bucket exists: {settings.minio_bucket}")
except Exception as e:
    print(f"⚠️ MinIO not available: {e}")
    minio_client = None


def get_minio():
    """获取 MinIO 客户端"""
    return minio_client


def upload_file(object_name: str, data: bytes, content_type: str = "application/octet-stream") -> str:
    """上传文件到 MinIO"""
    if minio_client is None:
        raise Exception("MinIO client not initialized")
    
    # 上传
    minio_client.put_object(
        settings.minio_bucket,
        object_name,
        io.BytesIO(data),
        length=len(data),
        content_type=content_type
    )
    
    # 返回 URL
    return f"http://{settings.minio_endpoint}/{settings.minio_bucket}/{object_name}"


def delete_file(object_name: str):
    """从 MinIO 删除文件"""
    if minio_client is None:
        raise Exception("MinIO client not initialized")
    
    minio_client.remove_object(settings.minio_bucket, object_name)


def get_file_url(object_name: str, expires: int = 3600) -> str:
    """获取文件预签名 URL"""
    if minio_client is None:
        raise Exception("MinIO client not initialized")
    
    return minio_client.presigned_get_object(settings.minio_bucket, object_name, expires)
