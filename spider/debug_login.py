#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
调试登录功能 - 直接调用后端API
"""

import requests
import json

# 后端API地址
API_URL = "http://localhost:9001/api/login"

# 测试凭据
username = "edidada"
password = "Pass123Qwe"

print(f"测试登录 API: {API_URL}")
print(f"用户名: {username}")
print(f"密码: {password}")

# 发送登录请求
payload = {
    "username": username,
    "password": password
}

print(f"\n发送的请求体: {json.dumps(payload, ensure_ascii=False)}")

response = requests.post(API_URL, json=payload, headers={"Content-Type": "application/json"})

print(f"\n响应状态码: {response.status_code}")
print(f"响应头: {dict(response.headers)}")
print(f"响应体: {response.json()}")

if response.status_code == 200:
    print("\n✓ 登录成功!")
else:
    print("\n✗ 登录失败!")
