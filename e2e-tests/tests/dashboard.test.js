const { test, expect } = require('@playwright/test');

test('仪表盘测试 - 健康检查', async ({ request }) => {
  // 测试健康检查接口
  const response = await request.get('http://localhost:9001/health');
  expect(response.status()).toBe(200);
  
  const data = await response.json();
  expect(data.status).toBe('healthy');
  expect(data.version).toBeTruthy();
});

test('仪表盘测试 - 访问字典管理', async ({ page }) => {
  // 先登录
  await page.goto('/');
  await page.fill('input[type="text"]', 'admin');
  await page.fill('input[type="password"]', '123456');
  await page.click('button:has-text("登录")');
  await page.waitForURL('**/dashboard');
  
  // 点击字典管理
  await page.click('text=字典管理');
  
  // 等待字典管理页面加载
  await page.waitForSelector('h2:has-text("字典管理")');
  
  // 验证页面标题
  await expect(page.locator('h2')).toHaveText('字典管理');
  
  // 验证表格存在
  await expect(page.locator('table')).toBeVisible();
});

test('仪表盘测试 - 访问用户管理', async ({ page }) => {
  // 先登录
  await page.goto('/');
  await page.fill('input[type="text"]', 'admin');
  await page.fill('input[type="password"]', '123456');
  await page.click('button:has-text("登录")');
  await page.waitForURL('**/dashboard');
  
  // 点击用户管理
  await page.click('text=用户管理');
  
  // 等待用户管理页面加载
  await page.waitForSelector('h2:has-text("用户管理")');
  
  // 验证页面标题
  await expect(page.locator('h2')).toHaveText('用户管理');
  
  // 验证表格存在
  await expect(page.locator('table')).toBeVisible();
});