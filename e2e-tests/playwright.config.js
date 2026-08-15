/** @type {import('@playwright/test').PlaywrightTestConfig} */
const config = {
  testDir: './tests',
  timeout: 30 * 1000,
  expect: {
    timeout: 5000
  },
  webServer: {
    command: 'npm run dev',
    port: 5174,
    cwd: '../frontend',
    reuseExistingServer: !process.env.CI,
  },
  reporter: [
    ['html'],
    ['json', { outputFile: 'test-results/results.json' }]
  ],
  use: {
    baseURL: 'http://localhost:5174',
    browserName: 'chromium',
    headless: false,
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
    // 使用手动下载的 Chrome
    executablePath: "/Users/ibqo/development/chrome-mac-x64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing"
  },
};

module.exports = config;