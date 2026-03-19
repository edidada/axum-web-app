#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Playwright 自动化测试 - 登录功能测试
测试目标: http://localhost:5173 (Vue前端)
后端API: http://localhost:9001 (Axum后端)

安装依赖:
    pip install playwright
    playwright install chromium
"""

import asyncio
import sys

# 检查 playwright 是否安装
try:
    from playwright.async_api import async_playwright
    print("✓ Playwright 导入成功")
except ImportError:
    print("✗ Playwright 未安装")
    print("请运行以下命令安装:")
    print("  pip install playwright")
    print("  playwright install chromium")
    sys.exit(1)


async def test_login():
    """测试登录功能"""
    print("\n=== 开始登录功能测试 ===\n")
    
    base_url = "http://localhost:5173"
    
    async with async_playwright() as p:
        # 启动浏览器
        browser = await p.chromium.launch(headless=False)  # headless=True 为无头模式
        context = await browser.new_context(viewport={'width': 1920, 'height': 1080})
        page = await context.new_page()
        
        try:
            # 测试1: 页面加载
            print("测试1: 验证登录页面加载...")
            await page.goto(f"{base_url}/login")
            await page.wait_for_load_state('networkidle')
            title = await page.title()
            print(f"  页面标题: {title}")
            print("  ✓ 登录页面加载成功")
            
            # 测试2: 查找表单元素
            print("\n测试2: 验证表单元素...")
            try:
                username_input = page.locator('#username')
                password_input = page.locator('#password')
                login_button = page.locator('button[type="submit"]')
                
                # 等待元素可见
                await username_input.wait_for(state='visible')
                await password_input.wait_for(state='visible')
                await login_button.wait_for(state='visible')
                
                print("  ✓ 找到用户名输入框")
                print("  ✓ 找到密码输入框")
                print("  ✓ 找到登录按钮")
            except Exception as e:
                print(f"  ✗ 查找表单元素失败: {e}")
                await browser.close()
                return
            
            # 测试3: 空表单提交
            print("\n测试3: 验证空表单提交...")
            await login_button.click()
            await asyncio.sleep(1)
            
            current_url = page.url
            if "/login" in current_url:
                print("  ✓ 空表单未跳转，停留在登录页")
            else:
                print(f"  ⚠ 当前URL: {current_url}")
            
            # 测试4: 错误凭据
            print("\n测试4: 验证错误凭据...")
            await page.goto(f"{base_url}/login")
            await page.wait_for_load_state('networkidle')
            
            await page.fill('#username', 'wronguser')
            await page.fill('#password', 'wrongpass')
            await page.click('button[type="submit"]')
            await asyncio.sleep(2)
            
            current_url = page.url
            if "/login" in current_url:
                print("  ✓ 错误凭据登录失败，停留在登录页")
            else:
                print(f"  ⚠ 当前URL: {current_url}")
            
            # 测试5: 正确登录
            print("\n测试5: 验证正确登录...")
            await page.goto(f"{base_url}/login")
            await page.wait_for_load_state('networkidle')
            
            await page.fill('#username', 'edidada')
            await page.fill('#password', 'Pass123Qwe')
            await page.click('button[type="submit"]')
            
            # 等待导航完成
            try:
                await page.wait_for_url("**/dashboard", timeout=5000)
                print("  ✓ 登录成功，跳转到仪表板")
                print(f"  当前URL: {page.url}")
            except:
                current_url = page.url
                print(f"  ✗ 登录失败，当前URL: {current_url}")
                # 截图保存
                await page.screenshot(path='login_failed.png')
                print("  截图已保存: login_failed.png")
            
            # 测试6: 菜单导航（如果登录成功）
            if "/dashboard" in page.url:
                print("\n测试6: 验证菜单导航...")
                menu_items = [
                    ("字典管理", "dict"),
                    ("地址簿", "address"),
                    ("分类管理", "category"),
                    ("菜品管理", "dish"),
                ]
                
                for idx, (label, expected_id) in enumerate(menu_items):
                    try:
                        # 查找菜单按钮
                        menu_button = page.locator(f'button:has-text("{label}")')
                        if await menu_button.count() > 0:
                            await menu_button.click()
                            await asyncio.sleep(1)
                            print(f"  ✓ 点击'{label}'菜单成功")
                        else:
                            print(f"  ⚠ 未找到'{label}'菜单")
                    except Exception as e:
                        print(f"  ✗ 点击'{label}'菜单失败: {e}")
            
            print("\n=== 测试完成 ===")
            
        except Exception as e:
            print(f"\n✗ 测试过程中发生错误: {e}")
            import traceback
            traceback.print_exc()
        finally:
            await asyncio.sleep(2)
            await browser.close()
            print("\n✓ 浏览器已关闭")


if __name__ == "__main__":
    asyncio.run(test_login())
