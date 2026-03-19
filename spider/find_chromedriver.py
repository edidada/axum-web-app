#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
查找指定版本的ChromeDriver下载链接
"""

import json

# 读取版本信息文件
with open('versions_with_downloads.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

# 查找145版本的ChromeDriver
for version in data['versions']:
    v = version['version']
    if v.startswith('145.'):
        print(f"找到版本: {v}")
        if 'chromedriver' in version['downloads']:
            for item in version['downloads']['chromedriver']:
                if item['platform'] == 'win64':
                    print(f"ChromeDriver下载链接: {item['url']}")
                    exit(0)

print("未找到145版本的ChromeDriver")
