#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Selenium 自动化测试 - 登录功能测试 (简化版)
测试目标: http://localhost:5173 (Vue前端)
后端API: http://localhost:9001 (Axum后端)
"""

import os
import time

# 尝试导入selenium
try:
    from selenium import webdriver
    from selenium.webdriver.common.by import By
    from selenium.webdriver.chrome.options import Options
    from selenium.webdriver.chrome.service import Service
    print("✓ Selenium 导入成功")
except ImportError as e:
    print(f"✗ Selenium 导入失败: {e}")
    print("请尝试运行: pip install selenium")
    exit(1)

def test_login():
    """测试登录功能"""
    print("\n=== 开始登录功能测试 ===\n")
    
    # 设置 Chrome 选项
    chrome_options = Options()
    chrome_options.add_argument('--no-sandbox')
    chrome_options.add_argument('--disable-dev-shm-usage')
    chrome_options.add_argument('--window-size=1920,1080')
    # 无头模式（不显示浏览器窗口）
    # chrome_options.add_argument('--headless')
    
    # 查找 chromedriver
    chromedriver_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "drivers", "chromedriver.exe")
    if not os.path.exists(chromedriver_path):
        chromedriver_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "drivers", "chromedriver")
    
    print(f"使用 chromedriver: {chromedriver_path}")
    
    driver = None
    try:
        # 创建 WebDriver
        driver = webdriver.Chrome(
            service=Service(chromedriver_path),
            options=chrome_options
        )
        driver.implicitly_wait(10)
        base_url = "http://localhost:5173"
        
        # 测试1: 页面加载
        print("\n测试1: 验证登录页面加载...")
        driver.get(f"{base_url}/login")
        time.sleep(2)
        print(f"  页面标题: {driver.title}")
        print("  ✓ 登录页面加载成功")
        
        # 测试2: 查找表单元素
        print("\n测试2: 验证表单元素...")
        try:
            username_input = driver.find_element(By.ID, "username")
            password_input = driver.find_element(By.ID, "password")
            login_button = driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
            print("  ✓ 找到用户名输入框")
            print("  ✓ 找到密码输入框")
            print("  ✓ 找到登录按钮")
        except Exception as e:
            print(f"  ✗ 查找表单元素失败: {e}")
            return
        
        # 测试3: 空表单提交
        print("\n测试3: 验证空表单提交...")
        login_button.click()
        time.sleep(1)
        current_url = driver.current_url
        if "/login" in current_url:
            print("  ✓ 空表单未跳转，停留在登录页")
        else:
            print(f"  ⚠ 当前URL: {current_url}")
        
        # 测试4: 错误凭据
        print("\n测试4: 验证错误凭据...")
        driver.get(f"{base_url}/login")
        time.sleep(1)
        username_input = driver.find_element(By.ID, "username")
        password_input = driver.find_element(By.ID, "password")
        login_button = driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
        
        username_input.send_keys("wronguser")
        password_input.send_keys("wrongpass")
        login_button.click()
        time.sleep(2)
        
        current_url = driver.current_url
        if "/login" in current_url:
            print("  ✓ 错误凭据登录失败，停留在登录页")
        else:
            print(f"  ⚠ 当前URL: {current_url}")
        
        # 测试5: 正确登录
        print("\n测试5: 验证正确登录...")
        driver.get(f"{base_url}/login")
        time.sleep(1)
        username_input = driver.find_element(By.ID, "username")
        password_input = driver.find_element(By.ID, "password")
        login_button = driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
        
        username_input.send_keys("edidada")
        password_input.send_keys("Pass123Qwe")
        login_button.click()
        time.sleep(3)
        
        current_url = driver.current_url
        if "/dashboard" in current_url:
            print("  ✓ 登录成功，跳转到仪表板")
            print(f"  当前URL: {current_url}")
        else:
            print(f"  ✗ 登录失败，当前URL: {current_url}")
        
        print("\n=== 测试完成 ===")
        
    except Exception as e:
        print(f"\n✗ 测试过程中发生错误: {e}")
        import traceback
        traceback.print_exc()
    finally:
        if driver:
            time.sleep(2)
            driver.quit()
            print("\n✓ 浏览器已关闭")

if __name__ == "__main__":
    test_login()
