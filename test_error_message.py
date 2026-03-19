#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
测试错误登录提示是否为中文
"""

import requests
import json

API_URL = "http://localhost:9001"

def test_invalid_login():
    """测试错误登录"""
    print("=" * 60)
    print("测试错误登录提示")
    print("=" * 60)
    
    # 测试1: 错误的用户名和密码
    print("\n测试1: 错误的用户名和密码")
    response = requests.post(
        f"{API_URL}/api/login",
        json={"username": "wronguser", "password": "wrongpass"}
    )
    data = response.json()
    print(f"状态码: {response.status_code}")
    print(f"响应数据: {json.dumps(data, ensure_ascii=False, indent=2)}")
    
    if "message" in data:
        print(f"\n错误提示: {data['message']}")
        if data['message'] == "用户名或密码错误":
            print("✓ 测试通过: 错误提示为中文")
        else:
            print("✗ 测试失败: 错误提示不是中文")
    
    # 测试2: 空用户名
    print("\n" + "=" * 60)
    print("测试2: 空用户名")
    response = requests.post(
        f"{API_URL}/api/login",
        json={"username": "", "password": "somepass"}
    )
    data = response.json()
    print(f"状态码: {response.status_code}")
    print(f"响应数据: {json.dumps(data, ensure_ascii=False, indent=2)}")
    
    if "message" in data:
        print(f"\n错误提示: {data['message']}")
    
    # 测试3: 空密码
    print("\n" + "=" * 60)
    print("测试3: 空密码")
    response = requests.post(
        f"{API_URL}/api/login",
        json={"username": "someuser", "password": ""}
    )
    data = response.json()
    print(f"状态码: {response.status_code}")
    print(f"响应数据: {json.dumps(data, ensure_ascii=False, indent=2)}")
    
    if "message" in data:
        print(f"\n错误提示: {data['message']}")
    
    print("\n" + "=" * 60)
    print("测试完成")
    print("=" * 60)

if __name__ == "__main__":
    test_invalid_login()
