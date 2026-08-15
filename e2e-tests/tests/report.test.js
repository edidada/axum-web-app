const { test, expect } = require('@playwright/test');

test('报表测试 - 订单详情报表', async ({ request, page }) => {
  // 先登录获取会话
  await page.goto('/');
  await page.fill('input[type="text"]', 'admin');
  await page.fill('input[type="password"]', '123456');
  await page.click('button:has-text("登录")');
  await page.waitForURL('**/dashboard');
  
  // 提取会话 cookie
  const cookies = await page.context().cookies();
  const cookieHeader = cookies.map(c => `${c.name}=${c.value}`).join('; ');
  
  // 测试订单详情报表接口
  const response = await request.get('http://localhost:9001/report/order-details', {
    headers: {
      'Cookie': cookieHeader
    }
  });
  
  expect(response.status()).toBe(200);
  
  const data = await response.json();
  expect(data.code).toBe(200);
  expect(Array.isArray(data.data)).toBe(true);
});

test('报表测试 - 套餐菜品报表', async ({ request, page }) => {
  // 先登录获取会话
  await page.goto('/');
  await page.fill('input[type="text"]', 'admin');
  await page.fill('input[type="password"]', '123456');
  await page.click('button:has-text("登录")');
  await page.waitForURL('**/dashboard');
  
  // 提取会话 cookie
  const cookies = await page.context().cookies();
  const cookieHeader = cookies.map(c => `${c.name}=${c.value}`).join('; ');
  
  // 测试套餐菜品报表接口
  const response = await request.get('http://localhost:9001/report/setmeal-dishes', {
    headers: {
      'Cookie': cookieHeader
    }
  });
  
  expect(response.status()).toBe(200);
  
  const data = await response.json();
  expect(data.code).toBe(200);
  expect(Array.isArray(data.data)).toBe(true);
});