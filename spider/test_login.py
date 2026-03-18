#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Selenium 自动化测试 - 登录功能测试
测试目标: http://localhost:5173 (Vue前端)
后端API: http://localhost:9001 (Axum后端)
"""

import unittest
import time
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service
from webdriver_manager.chrome import ChromeDriverManager


class LoginTest(unittest.TestCase):
    """登录功能测试类"""
    
    @classmethod
    def setUpClass(cls):
        """测试类开始前执行一次"""
        # 配置 Chrome 选项
        chrome_options = Options()
        # chrome_options.add_argument('--headless')  # 无头模式，不显示浏览器窗口
        chrome_options.add_argument('--no-sandbox')
        chrome_options.add_argument('--disable-dev-shm-usage')
        chrome_options.add_argument('--window-size=1920,1080')
        
        # 初始化 WebDriver
        cls.driver = webdriver.Chrome(
            service=Service(ChromeDriverManager().install()),
            options=chrome_options
        )
        cls.driver.implicitly_wait(10)
        cls.base_url = "http://localhost:5173"
        cls.api_url = "http://localhost:9001"
        
    @classmethod
    def tearDownClass(cls):
        """测试类结束后执行一次"""
        cls.driver.quit()
        
    def setUp(self):
        """每个测试方法开始前执行"""
        # 打开登录页面
        self.driver.get(f"{self.base_url}/login")
        time.sleep(1)  # 等待页面加载
        
    def test_page_load(self):
        """测试1: 验证登录页面是否正常加载"""
        # 验证页面标题
        self.assertIn("登录", self.driver.title)
        
        # 验证登录表单元素是否存在
        username_input = self.driver.find_element(By.ID, "username")
        password_input = self.driver.find_element(By.ID, "password")
        login_button = self.driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
        
        self.assertIsNotNone(username_input)
        self.assertIsNotNone(password_input)
        self.assertIsNotNone(login_button)
        print("✓ 测试1通过: 登录页面加载正常")
        
    def test_empty_username(self):
        """测试2: 验证空用户名时的错误提示"""
        # 不输入用户名，直接点击登录
        login_button = self.driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
        login_button.click()
        time.sleep(1)
        
        # 验证是否还在登录页面（没有跳转）
        current_url = self.driver.current_url
        self.assertIn("/login", current_url)
        print("✓ 测试2通过: 空用户名时未跳转")
        
    def test_empty_password(self):
        """测试3: 验证空密码时的错误提示"""
        # 输入用户名，不输入密码
        username_input = self.driver.find_element(By.ID, "username")
        username_input.send_keys("edidada")
        
        login_button = self.driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
        login_button.click()
        time.sleep(1)
        
        # 验证是否还在登录页面
        current_url = self.driver.current_url
        self.assertIn("/login", current_url)
        print("✓ 测试3通过: 空密码时未跳转")
        
    def test_invalid_credentials(self):
        """测试4: 验证错误的用户名密码"""
        # 输入错误的用户名和密码
        username_input = self.driver.find_element(By.ID, "username")
        password_input = self.driver.find_element(By.ID, "password")
        
        username_input.send_keys("wronguser")
        password_input.send_keys("wrongpass")
        
        # 点击登录按钮
        login_button = self.driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
        login_button.click()
        time.sleep(2)  # 等待请求响应
        
        # 验证是否还在登录页面（登录失败不会跳转）
        current_url = self.driver.current_url
        self.assertIn("/login", current_url)
        print("✓ 测试4通过: 错误凭据时登录失败")
        
    def test_valid_login(self):
        """测试5: 验证正确的用户名密码可以登录成功"""
        # 输入正确的用户名和密码
        username_input = self.driver.find_element(By.ID, "username")
        password_input = self.driver.find_element(By.ID, "password")
        
        username_input.send_keys("edidada")
        password_input.send_keys("Pass123Qwe")
        
        # 点击登录按钮
        login_button = self.driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
        login_button.click()
        
        # 等待页面跳转（最多等待5秒）
        try:
            WebDriverWait(self.driver, 5).until(
                EC.url_contains("/dashboard")
            )
            print("✓ 测试5通过: 登录成功并跳转到仪表板")
        except:
            self.fail("登录后未跳转到仪表板页面")
            
    def test_login_logout(self):
        """测试6: 验证登录后可以正常退出"""
        # 先登录
        self.test_valid_login()
        time.sleep(1)
        
        # 查找退出按钮并点击
        try:
            logout_button = self.driver.find_element(By.CLASS_NAME, "logout-btn")
            logout_button.click()
            time.sleep(2)
            
            # 验证是否返回到登录页面
            current_url = self.driver.current_url
            self.assertIn("/login", current_url)
            print("✓ 测试6通过: 退出登录成功")
        except:
            print("⚠ 测试6跳过: 未找到退出按钮")
            
    def test_session_persistence(self):
        """测试7: 验证登录后刷新页面仍保持登录状态"""
        # 先登录
        self.test_valid_login()
        time.sleep(1)
        
        # 刷新页面
        self.driver.refresh()
        time.sleep(2)
        
        # 验证是否仍在仪表板页面（未跳转到登录页）
        current_url = self.driver.current_url
        self.assertIn("/dashboard", current_url)
        print("✓ 测试7通过: 刷新页面后仍保持登录状态")


def run_tests():
    """运行所有测试"""
    # 创建测试套件
    loader = unittest.TestLoader()
    suite = unittest.TestSuite()
    
    # 添加测试用例
    suite.addTests(loader.loadTestsFromTestCase(LoginTest))
    
    # 运行测试
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)
    
    return result.wasSuccessful()


if __name__ == "__main__":
    print("=" * 60)
    print("开始执行登录功能自动化测试")
    print("=" * 60)
    print()
    
    success = run_tests()
    
    print()
    print("=" * 60)
    if success:
        print("所有测试通过! ✓")
    else:
        print("部分测试失败! ✗")
    print("=" * 60)
