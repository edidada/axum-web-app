from fastapi import APIRouter, UploadFile, File
from models.schemas import Response, ImageUploadRequest
import httpx
import io
from utils.minio_client import upload_file, delete_file, get_file_url
from utils.id_generator import generate_id_with_prefix

router = APIRouter()


@router.post("/image/upload-from-url")
async def upload_image_from_url(request: ImageUploadRequest):
    """从 URL 上传图片"""
    try:
        # 下载图片
        async with httpx.AsyncClient() as client:
            response = await client.get(request.url, timeout=30.0)
            response.raise_for_status()
            
        # 获取内容类型
        content_type = response.headers.get("content-type", "image/jpeg")
        
        # 生成文件名
        ext = ".jpg"
        if "png" in content_type:
            ext = ".png"
        elif "gif" in content_type:
            ext = ".gif"
        elif "webp" in content_type:
            ext = ".webp"
        
        object_name = f"images/{generate_id_with_prefix('IMG')}{ext}"
        
        # 上传到 MinIO
        url = upload_file(object_name, response.content, content_type)
        
        return Response(data={"url": url, "object_name": object_name})
    except Exception as e:
        return Response(code=1, status="error", message=str(e))


@router.delete("/image/{object_name}")
async def delete_image(object_name: str):
    """删除图片"""
    try:
        delete_file(object_name)
        return Response(message="删除成功")
    except Exception as e:
        return Response(code=1, status="error", message=str(e))


@router.get("/image/url/{object_name}")
async def get_image_url(object_name: str, expires: int = 3600):
    """获取图片 URL"""
    try:
        url = get_file_url(object_name, expires)
        return Response(data={"url": url})
    except Exception as e:
        return Response(code=1, status="error", message=str(e))
