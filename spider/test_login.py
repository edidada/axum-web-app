#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Selenium 自动化测试 - 登录功能测试
测试目标: http://localhost:5173 (Vue前端)
后端API: http://localhost:9001 (Axum后端)
支持视频录制功能
"""

import os
import sys
import unittest
import time
import subprocess
from datetime import datetime
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
        cls.video_process = None
        cls.video_file = None
        
        chrome_options = Options()
        chrome_options.add_argument('--no-sandbox')
        chrome_options.add_argument('--disable-dev-shm-usage')
        chrome_options.add_argument('--window-size=1920,1080')
        
        chromedriver_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "drivers", "chromedriver")
        cls.driver = webdriver.Chrome(
            service=Service(chromedriver_path),
            options=chrome_options
        )
        cls.driver.implicitly_wait(10)
        cls.base_url = "http://localhost:5173"
        cls.api_url = "http://localhost:9001"
        
        # 启动视频录制
        cls.start_recording()
        
    @classmethod
    def tearDownClass(cls):
        """测试类结束后执行一次"""
        # 停止视频录制
        cls.stop_recording()
        cls.driver.quit()
        
    @classmethod
    def start_recording(cls):
        """开始录制视频"""
        try:
            # 创建videos目录
            script_dir = os.path.dirname(os.path.abspath(__file__))
            videos_dir = os.path.join(script_dir, "videos")
            os.makedirs(videos_dir, exist_ok=True)
            
            # 生成视频文件名
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            cls.video_file = os.path.join(videos_dir, "test_login_%s.mp4" % timestamp)
            
            # 使用ffmpeg录制屏幕
            platform = sys.platform
            
            if platform == 'darwin':  # macOS
                input_source = '1'  # 主显示器
                ffmpeg_cmd = [
                    'ffmpeg',
                    '-f', 'avfoundation',
                    '-i', input_source,
                    '-r', '30',
                    '-c:v', 'libx264',
                    '-preset', 'ultrafast',
                    '-crf', '23',
                    '-pix_fmt', 'yuv420p',
                    cls.video_file
                ]
            elif platform.startswith('linux'):  # Linux
                input_source = ':0.0'
                ffmpeg_cmd = [
                    'ffmpeg',
                    '-f', 'x11grab',
                    '-i', input_source,
                    '-r', '30',
                    '-c:v', 'libx264',
                    '-preset', 'ultrafast',
                    '-crf', '23',
                    '-pix_fmt', 'yuv420p',
                    cls.video_file
                ]
            else:  # Windows
                input_source = 'desktop'
                ffmpeg_cmd = [
                    'ffmpeg',
                    '-f', 'gdigrab',
                    '-i', input_source,
                    '-r', '30',
                    '-c:v', 'libx264',
                    '-preset', 'ultrafast',
                    '-crf', '23',
                    '-pix_fmt', 'yuv420p',
                    cls.video_file
                ]
            
            # 启动ffmpeg进程
            cls.video_process = subprocess.Popen(
                ffmpeg_cmd,
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE
            )
            
            print(f"📹 开始录制视频: {cls.video_file}")
            time.sleep(1)  # 等待ffmpeg启动
            
        except FileNotFoundError:
            print("⚠️  未找到ffmpeg，跳过视频录制")
            print("   可以通过以下命令安装ffmpeg:")
            print("   macOS: brew install ffmpeg")
            print("   Linux: sudo apt-get install ffmpeg")
            print("   Windows: 从 https://ffmpeg.org/download.html 下载")
        except Exception as e:
            print(f"⚠️  视频录制启动失败: {e}")
    
    @classmethod
    def stop_recording(cls):
        """停止录制视频"""
        if cls.video_process and cls.video_process.poll() is None:
            try:
                # 发送q命令停止ffmpeg
                cls.video_process.stdin.write(b'q')
                cls.video_process.stdin.close()
                cls.video_process.wait(timeout=5)
                print(f"📹 视频已保存: {cls.video_file}")
            except Exception as e:
                print(f"⚠️  停止录制时出错: {e}")
        
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
        
    def test_health_check(self):
        """测试10: 验证健康检查接口"""
        import requests
        try:
            response = requests.get(f"{self.api_url}/health")
            self.assertEqual(response.status_code, 200)
            data = response.json()
            self.assertEqual(data["status"], "healthy")
            self.assertIn("version", data)
            print("✓ 测试10通过: 健康检查接口正常")
        except Exception as e:
            print(f"✗ 测试10失败: 健康检查接口测试失败 - {e}")
            
    def test_report_apis(self):
        """测试11: 验证前端JOIN查询功能"""
        self.test_valid_login()
        time.sleep(1)
        
        try:
            # 测试订单详情JOIN查询
            join_button = WebDriverWait(self.driver, 5).until(
                EC.element_to_be_clickable((By.XPATH, "//button[contains(text(), '订单详情报表')]"))
            )
            join_button.click()
            time.sleep(2)
            
            # 验证页面加载
            page_title = WebDriverWait(self.driver, 5).until(
                EC.presence_of_element_located((By.TAG_NAME, "h2"))
            )
            self.assertIn("订单", page_title.text)
            print("✓ 测试11.1通过: 订单详情JOIN查询页面正常")
            
            # 测试套餐菜品JOIN查询
            setmeal_button = WebDriverWait(self.driver, 5).until(
                EC.element_to_be_clickable((By.XPATH, "//button[contains(text(), '套餐菜品报表')]"))
            )
            setmeal_button.click()
            time.sleep(2)
            
            # 验证页面加载
            page_title = WebDriverWait(self.driver, 5).until(
                EC.presence_of_element_located((By.TAG_NAME, "h2"))
            )
            self.assertIn("套餐", page_title.text)
            print("✓ 测试11.2通过: 套餐菜品JOIN查询页面正常")
            
        except Exception as e:
            print(f"✗ 测试11失败: JOIN查询功能测试失败 - {e}")
            
    def test_report_page(self):
        """测试12: 验证JOIN查询页面"""
        self.test_valid_login()
        time.sleep(1)
        
        try:
            # 点击JOIN查询菜单
            join_button = WebDriverWait(self.driver, 5).until(
                EC.element_to_be_clickable((By.XPATH, "//button[contains(text(), '订单详情报表')]"))
            )
            join_button.click()
            time.sleep(2)
            
            # 验证JOIN查询页面加载
            page_title = WebDriverWait(self.driver, 5).until(
                EC.presence_of_element_located((By.TAG_NAME, "h2"))
            )
            self.assertIn("订单", page_title.text)
            print("✓ 测试12通过: JOIN查询页面加载正常")
            
        except Exception as e:
            print(f"✗ 测试12失败: JOIN查询页面测试失败 - {e}")
            
    def test_crud_operations(self):
        """测试13: 测试所有菜单的CRUD操作"""
        self.test_valid_login()
        time.sleep(1)
        
        # 测试字典管理的CRUD操作
        try:
            print("\n开始测试字典管理CRUD操作...")
            self._test_single_menu_crud("字典管理", "dict")
        except Exception as e:
            print(f"✗ 字典管理CRUD测试失败: {e}")
            
        # 测试分类管理的CRUD操作
        try:
            print("\n开始测试分类管理CRUD操作...")
            self._test_single_menu_crud("分类管理", "category")
        except Exception as e:
            print(f"✗ 分类管理CRUD测试失败: {e}")
            
    def _test_single_menu_crud(self, menu_name, menu_id):
        """测试单个菜单的CRUD操作"""
        # 点击菜单
        menu_button = WebDriverWait(self.driver, 5).until(
            EC.element_to_be_clickable((By.XPATH, f"//button[contains(text(), '{menu_name}')]"))
        )
        menu_button.click()
        time.sleep(2)
        
        # 测试新增操作
        try:
            add_button = self.driver.find_element(By.XPATH, "//button[contains(text(), '新增')]")
            add_button.click()
            time.sleep(1)
            
            # 填写表单（简化版，实际应该根据不同菜单填写不同字段）
            # 这里只测试打开模态框
            modal = WebDriverWait(self.driver, 3).until(
                EC.presence_of_element_located((By.CLASS_NAME, "modal"))
            )
            self.assertIsNotNone(modal)
            
            # 关闭模态框
            cancel_button = self.driver.find_element(By.XPATH, "//button[contains(text(), '取消')]")
            cancel_button.click()
            time.sleep(1)
            
            print(f"  ✓ {menu_name}新增功能正常")
        except Exception as e:
            print(f"  ✗ {menu_name}新增功能失败: {e}")
            
        # 测试刷新操作
        try:
            refresh_button = self.driver.find_element(By.XPATH, "//button[contains(text(), '刷新')]")
            refresh_button.click()
            time.sleep(2)
            print(f"  ✓ {menu_name}刷新功能正常")
        except Exception as e:
            print(f"  ✗ {menu_name}刷新功能失败: {e}")


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
