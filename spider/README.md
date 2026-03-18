# Selenium 自动化测试

## 简介

使用 Selenium WebDriver 对 axum-web-app 项目进行自动化测试，主要测试登录功能。

## 环境要求

- Python 3.8+
- Chrome 浏览器
- ChromeDriver (会自动下载)

## 安装依赖

```bash
cd spider
pip install -r requirements.txt
```

## 运行测试

### 1. 确保后端和前端服务已启动

```bash
# 终端1: 启动后端
cargo run --bin main

# 终端2: 启动前端
cd frontend
npm run dev
```

### 2. 运行测试脚本

```bash
# 运行所有测试
python test_login.py

# 使用 pytest 运行
pytest test_login.py -v
```

## 测试用例

1. **test_page_load** - 验证登录页面是否正常加载
2. **test_empty_username** - 验证空用户名时的错误提示
3. **test_empty_password** - 验证空密码时的错误提示
4. **test_invalid_credentials** - 验证错误的用户名密码
5. **test_valid_login** - 验证正确的用户名密码可以登录成功
6. **test_login_logout** - 验证登录后可以正常退出
7. **test_session_persistence** - 验证登录后刷新页面仍保持登录状态

## 配置

测试默认访问：
- 前端地址: http://localhost:5173
- 后端地址: http://localhost:9001

如需修改，请编辑 `test_login.py` 文件中的 `base_url` 和 `api_url` 变量。

## 注意事项

1. 测试过程中会打开 Chrome 浏览器窗口
2. 如需无头模式（不显示浏览器），取消 `setUpClass` 方法中的 `chrome_options.add_argument('--headless')` 注释
3. 确保测试前数据库已正确配置且有测试数据
