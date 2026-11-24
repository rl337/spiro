import { test, expect } from "@playwright/test";

test("US-026 Chromium MNIST WASM predicts digit 1 on CPU", async ({ page }) => {
  await page.goto("/index.html");
  await page.waitForFunction(() => document.body.dataset.ready !== undefined, null, {
    timeout: 120_000,
  });

  const ready = await page.evaluate(() => document.body.dataset.ready);
  const err = await page.evaluate(() => document.body.dataset.error || "");
  expect(ready, `wasm page failed: ${err}`).toBe("1");

  await expect(page.locator("body")).toHaveAttribute("data-digit", "1");
  await expect(page.locator("body")).toHaveAttribute("data-device", "cpu");

  const cpuMs = Number(await page.evaluate(() => document.body.dataset.cpuTimeMs));
  expect(cpuMs).toBeGreaterThan(0);

  await expect(page.locator("#status")).toContainText("predicted digit: 1");
});

test("US-026 preferred webgpu is rejected", async ({ page }) => {
  await page.goto("/index.html");
  // Run a small inline check once wasm is available from the main page module path.
  const message = await page.evaluate(async () => {
    const mod = await import("./pkg/uie_wasm.js");
    await mod.default();
    try {
      new mod.WasmInferenceEngine("webgpu");
      return "expected_throw";
    } catch (e) {
      return String(e);
    }
  });
  expect(message).toMatch(/webgpu_not_implemented/);
});
