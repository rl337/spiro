import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir: '.', testMatch: 'qwen.spec.mjs', timeout: 600_000,
  retries: 0, workers: 1,
  use: { baseURL: 'http://127.0.0.1:18766', headless: true },
  webServer: { command: 'node server.mjs', url: 'http://127.0.0.1:18766', reuseExistingServer: false, timeout: 30_000 },
  projects: [{ name: 'chromium', use: { browserName: 'chromium', launchOptions: { args: ['--disable-gpu'] } } }],
});
