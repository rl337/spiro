import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: ".",
  testMatch: /mnist\.spec\.mjs$/,
  timeout: 180_000,
  use: {
    baseURL: "http://127.0.0.1:18765",
    headless: true,
  },
  webServer: {
    command: "python3 -m http.server 18765 --bind 127.0.0.1",
    cwd: ".",
    url: "http://127.0.0.1:18765/index.html",
    reuseExistingServer: !process.env.CI,
    timeout: 60_000,
  },
  projects: [{ name: "chromium", use: { browserName: "chromium" } }],
});
