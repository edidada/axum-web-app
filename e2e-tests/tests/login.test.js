const { test, expect } = require('@playwright/test');

test('登录测试 - 成功登录', async ({ page, context }) => {
  // 导航到登录页
  await page.goto('/');
  
  // 等待登录表单加载
  await page.waitForSelector('input[type="text"]');
  
  // 输入用户名和密码
  await page.fill('input[type="text"]', 'admin');
  await page.fill('input[type="password"]', '123456');
  
  // 点击登录按钮
  await page.click('button:has-text("登录")');
  
  // 等待登录成功并跳转到首页
  await page.waitForURL('**/dashboard');
  
  // 验证是否成功登录（检查页面标题）
  await expect(page).toHaveTitle(/后台管理/);
  
  // 验证是否存在仪表盘元素
  await expect(page.locator('h1')).toHaveText(/仪表盘/);
});

test('登录测试 - 失败登录', async ({ page }) => {
  // 导航到登录页
  await page.goto('/');
  
  // 输入错误的用户名和密码
  await page.fill('input[type="text"]', 'wrong');
  await page.fill('input[type="password"]', 'wrong');
  
  // 点击登录按钮
  await page.click('button:has-text("登录")');
  
  // 等待错误提示
  await page.waitForSelector('.el-message');
  
  // 验证错误提示
  await expect(page.locator('.el-message')).toHaveText(/用户名或密码错误/);
});