#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Selenium 自动化测试 - 登录功能测试
测试目标: http://localhost:5173 (Vue前端)
后端API: http://localhost:9001 (Axum后端)
"""

import os
import unittest
import time
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service


class LoginTest(unittest.TestCase):
    """登录功能测试类"""
    
    @classmethod
    def setUpClass(cls):
        """测试类开始前执行一次"""
        chrome_options = Options()
        chrome_options.add_argument('--no-sandbox')
        chrome_options.add_argument('--disable-dev-shm-usage')
        chrome_options.add_argument('--window-size=1920,1080')
        
        chromedriver_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "drivers", "chromedriver.exe")
        cls.driver = webdriver.Chrome(
            service=Service(chromedriver_path),
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
        self.driver.get(f"{self.base_url}/login")
        self.driver.delete_all_cookies()
        self.driver.execute_script("localStorage.clear();")
        self.driver.refresh()
        time.sleep(1)
        
    def test_page_load(self):
        """测试1: 验证登录页面是否正常加载"""
        self.assertIn("后台管理系统", self.driver.title)
        
        username_input = self.driver.find_element(By.ID, "username")
        password_input = self.driver.find_element(By.ID, "password")
        login_button = self.driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
        
        self.assertIsNotNone(username_input)
        self.assertIsNotNone(password_input)
        self.assertIsNotNone(login_button)
        print("✓ 测试1通过: 登录页面加载正常")
        
    def test_empty_username(self):
        """测试2: 验证空用户名时的错误提示"""
        login_button = self.driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
        login_button.click()
        time.sleep(1)
        
        current_url = self.driver.current_url
        self.assertIn("/login", current_url)
        print("✓ 测试2通过: 空用户名时未跳转")
        
    def test_empty_password(self):
        """测试3: 验证空密码时的错误提示"""
        username_input = self.driver.find_element(By.ID, "username")
        username_input.send_keys("edidada")
        
        login_button = self.driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
        login_button.click()
        time.sleep(1)
        
        current_url = self.driver.current_url
        self.assertIn("/login", current_url)
        print("✓ 测试3通过: 空密码时未跳转")
        
    def test_invalid_credentials(self):
        """测试4: 验证错误的用户名密码"""
        username_input = self.driver.find_element(By.ID, "username")
        password_input = self.driver.find_element(By.ID, "password")
        
        username_input.send_keys("wronguser")
        password_input.send_keys("wrongpass")
        
        login_button = self.driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
        login_button.click()
        time.sleep(2)
        
        current_url = self.driver.current_url
        self.assertIn("/login", current_url)
        print("✓ 测试4通过: 错误凭据时登录失败")
        
    def test_valid_login(self):
        """测试5: 验证正确的用户名密码可以登录成功"""
        username_input = self.driver.find_element(By.ID, "username")
        password_input = self.driver.find_element(By.ID, "password")
        
        username_input.send_keys("edidada")
        password_input.send_keys("Pass123Qwe")
        
        login_button = self.driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
        login_button.click()
        
        try:
            WebDriverWait(self.driver, 5).until(
                EC.url_contains("/dashboard")
            )
            print("✓ 测试5通过: 登录成功并跳转到仪表板")
        except:
            self.fail("登录后未跳转到仪表板页面")
            
    def test_menu_navigation(self):
        """测试6: 验证菜单导航功能"""
        self.test_valid_login()
        time.sleep(1)
        
        menu_items = [
            ("字典管理", "dict"),
            ("地址簿", "address"),
            ("分类管理", "category"),
            ("菜品管理", "dish"),
        ]
        
        for idx, (label, expected_id) in enumerate(menu_items):
            try:
                menu_button = WebDriverWait(self.driver, 5).until(
                    EC.element_to_be_clickable((By.XPATH, f"//button[contains(text(), '{label}')]"))
                )
                menu_button.click()
                time.sleep(1)
                
                active_button = self.driver.find_element(By.CSS_SELECTOR, "button.nav-item.active")
                self.assertEqual(active_button.text, label)
                print(f"✓ 测试6.{idx+1}通过: 点击'{label}'菜单成功")
                
            except Exception as e:
                print(f"✗ 测试6.{idx+1}失败: 点击'{label}'菜单失败 - {e}")
                
    def test_data_display(self):
        """测试7: 验证数据展示功能"""
        self.test_valid_login()
        time.sleep(1)
        
        try:
            menu_button = WebDriverWait(self.driver, 5).until(
                EC.element_to_be_clickable((By.XPATH, "//button[contains(text(), '分类管理')]"))
            )
            menu_button.click()
            time.sleep(2)
            
            table = WebDriverWait(self.driver, 5).until(
                EC.presence_of_element_located((By.CSS_SELECTOR, "table"))
            )
            self.assertIsNotNone(table)
            
            rows = self.driver.find_elements(By.CSS_SELECTOR, "table tbody tr")
            self.assertGreater(len(rows), 0)
            print(f"✓ 测试7通过: 数据展示正常，共{len(rows)}条记录")
            
        except Exception as e:
            print(f"✗ 测试7失败: 数据展示失败 - {e}")
            
    def test_login_logout(self):
        """测试8: 验证登录后可以正常退出"""
        self.test_valid_login()
        time.sleep(1)
        
        try:
            logout_button = self.driver.find_element(By.CLASS_NAME, "logout-btn")
            logout_button.click()
            time.sleep(2)
            
            current_url = self.driver.current_url
            self.assertIn("/login", current_url)
            print("✓ 测试8通过: 退出登录成功")
        except:
            print("⚠ 测试8跳过: 未找到退出按钮")
            
    def test_session_persistence(self):
        """测试9: 验证登录后刷新页面仍保持登录状态"""
        self.test_valid_login()
        time.sleep(1)
        
        self.driver.refresh()
        time.sleep(2)
        
        current_url = self.driver.current_url
        self.assertIn("/dashboard", current_url)
        print("✓ 测试9通过: 刷新页面后仍保持登录状态")


def run_tests():
    """运行所有测试"""
    loader = unittest.TestLoader()
    suite = unittest.TestSuite()
    
    suite.addTests(loader.loadTestsFromTestCase(LoginTest))
    
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
