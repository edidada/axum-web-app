#!/usr/bin/env python3
"""
查询功能自动化测试

测试步骤：
1. 连接PostgreSQL数据库
2. 从数据库查询数据
3. 在网页上执行相同的查询
4. 对比数据库和网页的查询结果
5. 输出测试报告
"""

import unittest
import psycopg2
import time
import subprocess
import platform
import os
from datetime import datetime
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC


class SearchTest(unittest.TestCase):
    """查询功能测试类"""
    
    # 类级别的错误列表，用于收集所有错误
    errors = []
    video_process = None
    video_file = None
    
    @classmethod
    def setUpClass(cls):
        """测试类开始前执行一次"""
        # 初始化浏览器，使用项目目录下的 chromedriver
        from selenium.webdriver.chrome.service import Service
        import os
        chromedriver_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'drivers', 'chromedriver')
        service = Service(chromedriver_path)
        cls.driver = webdriver.Chrome(service=service)
        cls.driver.implicitly_wait(10)
        
        # 连接数据库
        cls.db_conn = cls.connect_database()
        
        # 开始录制视频
        cls.start_recording()
        
        # 登录
        cls.login()
    
    @classmethod
    def tearDownClass(cls):
        """测试类结束后执行一次"""
        # 打印错误汇总
        if cls.errors:
            print("\n" + "="*80)
            print(f"❌ 查询测试失败汇总 - 共 {len(cls.errors)} 个错误")
            print("="*80)
            for i, error in enumerate(cls.errors, 1):
                print(f"\n【错误 {i}】")
                print(f"  菜单: {error['menu']}")
                print(f"  查询条件: {error['search_params']}")
                print(f"  数据库记录数: {error['db_count']}")
                print(f"  网页记录数: {error['web_count']}")
                print(f"  错误详情: {error['message']}")
                if 'db_data' in error:
                    print(f"  数据库数据: {error['db_data']}")
                if 'web_data' in error:
                    print(f"  网页数据: {error['web_data']}")
        else:
            print("\n✅ 所有查询测试通过！")
        
        # 停止视频录制
        cls.stop_recording()
        cls.driver.quit()
        
        # 关闭数据库连接
        if cls.db_conn:
            cls.db_conn.close()
    
    @classmethod
    def connect_database(cls):
        """连接PostgreSQL数据库"""
        try:
            conn = psycopg2.connect(
                host="localhost",
                port="5432",
                database="ruiji",
                user="ibqo"
            )
            print("✅ 数据库连接成功")
            return conn
        except Exception as e:
            print(f"❌ 数据库连接失败: {e}")
            return None
    
    @classmethod
    def start_recording(cls):
        """开始录制视频"""
        try:
            # 检查ffmpeg是否安装
            ffmpeg_path = subprocess.run(['which', 'ffmpeg'], capture_output=True, text=True).stdout.strip()
            if not ffmpeg_path:
                print("⚠️  未找到ffmpeg，跳过视频录制")
                return
            
            # 创建视频保存目录
            video_dir = os.path.join(os.path.dirname(__file__), 'videos')
            os.makedirs(video_dir, exist_ok=True)
            
            # 生成视频文件名
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            cls.video_file = os.path.join(video_dir, f'test_search_{timestamp}.mp4')
            
            # 根据操作系统选择录制参数
            system = platform.system()
            if system == 'Darwin':  # macOS
                input_source = '1'  # 通常1是屏幕
                ffmpeg_cmd = [
                    ffmpeg_path,
                    '-f', 'avfoundation',
                    '-i', input_source,
                    '-r', '30',
                    '-c:v', 'libx264',
                    '-preset', 'ultrafast',
                    '-crf', '23',
                    '-pix_fmt', 'yuv420p',
                    cls.video_file
                ]
            elif platform.startswith('linux'):
                input_source = ':0.0'
                ffmpeg_cmd = [
                    ffmpeg_path,
                    '-f', 'x11grab',
                    '-i', input_source,
                    '-r', '30',
                    '-c:v', 'libx264',
                    '-preset', 'ultrafast',
                    '-crf', '23',
                    '-pix_fmt', 'yuv420p',
                    cls.video_file
                ]
            else:
                input_source = 'desktop'
                ffmpeg_cmd = [
                    ffmpeg_path,
                    '-f', 'gdigrab',
                    '-i', input_source,
                    '-r', '30',
                    '-c:v', 'libx264',
                    '-preset', 'ultrafast',
                    '-crf', '23',
                    '-pix_fmt', 'yuv420p',
                    cls.video_file
                ]
            
            cls.video_process = subprocess.Popen(
                ffmpeg_cmd,
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE
            )
            
            print(f"📹 开始录制视频: {cls.video_file}")
            time.sleep(1)
            
        except FileNotFoundError:
            print("⚠️  未找到ffmpeg，跳过视频录制")
        except Exception as e:
            print(f"⚠️  视频录制启动失败: {e}")
    
    @classmethod
    def stop_recording(cls):
        """停止录制视频"""
        if cls.video_process:
            try:
                # 发送q键停止录制
                cls.video_process.stdin.write(b'q')
                cls.video_process.stdin.flush()
                cls.video_process.wait(timeout=5)
                print(f"📹 视频已保存: {cls.video_file}")
            except Exception as e:
                print(f"⚠️  视频录制停止失败: {e}")
                cls.video_process.terminate()
    
    @classmethod
    def login(cls):
        """登录系统"""
        print("🌐 正在打开登录页面...")
        cls.driver.get("http://localhost:5173/login")
        time.sleep(2)
        
        print("🔍 查找用户名输入框...")
        # 等待页面加载完成
        WebDriverWait(cls.driver, 10).until(
            EC.presence_of_element_located((By.ID, "username"))
        )
        
        # 输入用户名
        username_input = cls.driver.find_element(By.ID, "username")
        username_input.clear()
        username_input.send_keys("edidada")
        print("✅ 用户名已输入")
        
        # 输入密码
        print("🔍 查找密码输入框...")
        password_input = cls.driver.find_element(By.ID, "password")
        password_input.clear()
        password_input.send_keys("Pass123Qwe")
        print("✅ 密码已输入")
        
        # 点击登录按钮
        print("🔍 查找登录按钮...")
        login_button = cls.driver.find_element(By.CSS_SELECTOR, "button[type='submit']")
        login_button.click()
        print("✅ 登录按钮已点击")
        
        print("⏳ 等待页面跳转...")
        time.sleep(3)
        current_url = cls.driver.current_url
        print(f"📍 当前URL: {current_url}")
        
        # 检查是否在 dashboard 页面
        if "/dashboard" not in current_url:
            # 检查是否有错误消息
            try:
                error_msg = cls.driver.find_element(By.CSS_SELECTOR, ".error-message").text
                print(f"❌ 登录错误: {error_msg}")
            except:
                print(f"⚠️  页面内容: {cls.driver.page_source[:500]}")
        
        WebDriverWait(cls.driver, 10).until(
            EC.url_contains("/dashboard")
        )
        print("✅ 登录成功，已进入仪表盘")
        time.sleep(1)
    
    def get_db_data(self, table_name, search_field=None, search_value=None):
        """从数据库获取数据"""
        if not self.db_conn:
            return []
        
        cursor = self.db_conn.cursor()
        try:
            # 先检查表是否存在
            cursor.execute("""
                SELECT EXISTS (
                    SELECT FROM information_schema.tables 
                    WHERE table_name = %s
                );
            """, (table_name,))
            exists = cursor.fetchone()[0]
            if not exists:
                print(f"⚠️  表 {table_name} 不存在")
                return []
            
            # 检查是否有 is_deleted 字段
            cursor.execute("""
                SELECT EXISTS (
                    SELECT FROM information_schema.columns 
                    WHERE table_name = %s AND column_name = 'is_deleted'
                );
            """, (table_name,))
            has_is_deleted = cursor.fetchone()[0]
            
            # 构建查询
            if search_field and search_value:
                # 模糊查询
                if has_is_deleted:
                    query = f"SELECT * FROM {table_name} WHERE {search_field} LIKE %s AND is_deleted = 0"
                else:
                    query = f"SELECT * FROM {table_name} WHERE {search_field} LIKE %s"
                cursor.execute(query, (f'%{search_value}%',))
            else:
                if has_is_deleted:
                    query = f"SELECT * FROM {table_name} WHERE is_deleted = 0"
                else:
                    query = f"SELECT * FROM {table_name}"
                cursor.execute(query)
            
            columns = [desc[0] for desc in cursor.description]
            rows = cursor.fetchall()
            
            result = []
            for row in rows:
                result.append(dict(zip(columns, row)))
            return result
        except Exception as e:
            print(f"数据库查询错误: {e}")
            return []
        finally:
            cursor.close()
    
    def get_web_table_data(self):
        """获取网页表格数据"""
        try:
            table = WebDriverWait(self.driver, 5).until(
                EC.presence_of_element_located((By.CSS_SELECTOR, "table"))
            )
            
            # 获取表头
            headers = []
            header_cells = table.find_elements(By.CSS_SELECTOR, "thead th")
            for cell in header_cells:
                headers.append(cell.text.strip())
            
            # 获取数据行
            rows_data = []
            rows = table.find_elements(By.CSS_SELECTOR, "tbody tr")
            for row in rows:
                cells = row.find_elements(By.TAG_NAME, "td")
                if len(cells) > 0:
                    row_data = {}
                    for i, cell in enumerate(cells):
                        if i < len(headers):
                            row_data[headers[i]] = cell.text.strip()
                    rows_data.append(row_data)
            
            return rows_data
        except Exception as e:
            print(f"获取网页表格数据错误: {e}")
            return []
    
    def perform_search(self, menu_name, search_params):
        """执行查询操作"""
        try:
            # 点击菜单
            menu_items = self.driver.find_elements(By.CSS_SELECTOR, ".sidebar-menu .menu-item, .el-menu-item, .menu li")
            for item in menu_items:
                if menu_name in item.text:
                    item.click()
                    time.sleep(1)
                    break
            
            # 输入查询条件
            for field, value in search_params.items():
                try:
                    # 尝试多种选择器
                    selectors = [
                        f"input[placeholder*='{field}']",
                        f"input[name='{field}']",
                        f"input[id='{field}']",
                        f"input[type='text']",
                    ]
                    input_field = None
                    for selector in selectors:
                        try:
                            input_field = self.driver.find_element(By.CSS_SELECTOR, selector)
                            if input_field:
                                break
                        except:
                            continue
                    
                    if input_field:
                        input_field.clear()
                        input_field.send_keys(value)
                except Exception as e:
                    print(f"输入查询条件 {field} 失败: {e}")
            
            # 点击查询按钮
            search_button = None
            button_selectors = [
                "button:contains('查询')",
                "button[type='submit']",
                ".search-btn",
                "button.el-button--primary",
            ]
            for selector in button_selectors:
                try:
                    buttons = self.driver.find_elements(By.CSS_SELECTOR, selector.replace(':contains(', '[').replace(')', ']'))
                    for btn in buttons:
                        if '查询' in btn.text or '搜索' in btn.text:
                            search_button = btn
                            break
                    if search_button:
                        break
                except:
                    continue
            
            if search_button:
                search_button.click()
                time.sleep(2)  # 等待查询结果
                return True
            else:
                print("未找到查询按钮")
                return False
                
        except Exception as e:
            print(f"执行查询操作失败: {e}")
            return False
    
    def add_error(self, menu, search_params, db_count, web_count, message, db_data=None, web_data=None):
        """添加错误记录"""
        error = {
            'menu': menu,
            'search_params': search_params,
            'db_count': db_count,
            'web_count': web_count,
            'message': message,
            'db_data': db_data,
            'web_data': web_data
        }
        self.__class__.errors.append(error)
    
    # ==================== 测试方法 ====================
    
    def test_dict_search_by_code(self):
        """测试1: 字典管理按编码查询"""
        menu_name = "字典管理"
        table_name = "sys_dict"
        search_field = "code"
        search_value = "232"
        
        db_data = self.get_db_data(table_name, search_field, search_value)
        
        if self.perform_search(menu_name, {search_field: search_value}):
            web_data = self.get_web_table_data()
            
            db_count = len(db_data)
            web_count = len(web_data)
            
            if db_count != web_count:
                self.add_error(menu_name, {search_field: search_value}, db_count, web_count, 
                             f"记录数不匹配", db_data, web_data)
                self.fail(f"❌ {menu_name} 按{search_field}='{search_value}'查询: 数据库{db_count}条，网页{web_count}条")
            else:
                print(f"✅ {menu_name} 按{search_field}='{search_value}'查询通过: 共{db_count}条记录")
    
    def test_dict_search_by_name(self):
        """测试2: 字典管理按名称查询"""
        menu_name = "字典管理"
        table_name = "sys_dict"
        search_field = "name"
        search_value = "测试"
        
        db_data = self.get_db_data(table_name, search_field, search_value)
        
        if self.perform_search(menu_name, {search_field: search_value}):
            web_data = self.get_web_table_data()
            
            db_count = len(db_data)
            web_count = len(web_data)
            
            if db_count != web_count:
                self.add_error(menu_name, {search_field: search_value}, db_count, web_count,
                             f"记录数不匹配", db_data, web_data)
                self.fail(f"❌ {menu_name} 按{search_field}='{search_value}'查询: 数据库{db_count}条，网页{web_count}条")
            else:
                print(f"✅ {menu_name} 按{search_field}='{search_value}'查询通过: 共{db_count}条记录")
    
    def test_dish_search_by_name(self):
        """测试3: 菜品管理按名称查询"""
        menu_name = "菜品管理"
        table_name = "dish"
        search_field = "name"
        search_value = "菜"
        
        db_data = self.get_db_data(table_name, search_field, search_value)
        
        if self.perform_search(menu_name, {search_field: search_value}):
            web_data = self.get_web_table_data()
            
            db_count = len(db_data)
            web_count = len(web_data)
            
            if db_count != web_count:
                self.add_error(menu_name, {search_field: search_value}, db_count, web_count,
                             f"记录数不匹配", db_data, web_data)
                self.fail(f"❌ {menu_name} 按{search_field}='{search_value}'查询: 数据库{db_count}条，网页{web_count}条")
            else:
                print(f"✅ {menu_name} 按{search_field}='{search_value}'查询通过: 共{db_count}条记录")
    
    def test_category_search_by_name(self):
        """测试4: 分类管理按名称查询"""
        menu_name = "分类管理"
        table_name = "category"
        search_field = "name"
        search_value = "类"
        
        db_data = self.get_db_data(table_name, search_field, search_value)
        
        if self.perform_search(menu_name, {search_field: search_value}):
            web_data = self.get_web_table_data()
            
            db_count = len(db_data)
            web_count = len(web_data)
            
            if db_count != web_count:
                self.add_error(menu_name, {search_field: search_value}, db_count, web_count,
                             f"记录数不匹配", db_data, web_data)
                self.fail(f"❌ {menu_name} 按{search_field}='{search_value}'查询: 数据库{db_count}条，网页{web_count}条")
            else:
                print(f"✅ {menu_name} 按{search_field}='{search_value}'查询通过: 共{db_count}条记录")
    
    def test_address_book_search_by_consignee(self):
        """测试5: 地址簿按收货人查询"""
        menu_name = "地址簿管理"
        table_name = "address_book"
        search_field = "consignee"
        search_value = "张"
        
        db_data = self.get_db_data(table_name, search_field, search_value)
        
        if self.perform_search(menu_name, {search_field: search_value}):
            web_data = self.get_web_table_data()
            
            db_count = len(db_data)
            web_count = len(web_data)
            
            if db_count != web_count:
                self.add_error(menu_name, {search_field: search_value}, db_count, web_count,
                             f"记录数不匹配", db_data, web_data)
                self.fail(f"❌ {menu_name} 按{search_field}='{search_value}'查询: 数据库{db_count}条，网页{web_count}条")
            else:
                print(f"✅ {menu_name} 按{search_field}='{search_value}'查询通过: 共{db_count}条记录")


if __name__ == "__main__":
    unittest.main(verbosity=2)
