import { test, expect } from '@playwright/test';
import { randomInt, createHash } from 'node:crypto';
import { createReadStream } from 'node:fs';
import { stat, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { homedir } from 'node:os';

test('US-026 Qwen 0.6B offline browser CPU multi-turn chat', async ({ page, context, browser }, testInfo) => {
  const errors = [], requests = [], turns = [];
  const modelPath = process.env.SPIRO_QWEN_MODEL ?? resolve(homedir(), '.lmstudio/models/Qwen/Qwen3-0.6B-GGUF/Qwen3-0.6B-Q8_0.gguf');
  const hash = createHash('sha256');
  for await (const part of createReadStream(modelPath)) hash.update(part);
  const evidence = { runtime: '@wllama/wllama@4.0.0', browser: browser.version(), model: { file: modelPath, bytes: (await stat(modelPath)).size, sha256: hash.digest('hex') }, turns, errors, requests };
  page.on('pageerror', error => errors.push(String(error)));
  page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  page.on('request', req => requests.push({ method: req.method(), url: req.url() }));
  try {
    await page.goto('/');
    await page.waitForFunction(() => document.body.dataset.ready !== undefined, null, { timeout: 240_000 });
    expect(await page.locator('body').getAttribute('data-error')).toBeNull();
    expect(await page.locator('body').getAttribute('data-ready')).toBe('1');
    Object.assign(evidence, await page.evaluate(() => ({ loadMs: qwen.loadMs, settings: qwen.settings, generation: qwen.generation, context: qwen.info, multithread: qwen.multithread, isolated: crossOriginIsolated })));
    expect(evidence.isolated).toBe(true);
    expect(evidence.settings.n_gpu_layers).toBe(0);
    expect(evidence.context.metadata['general.architecture']).toBe('qwen3');
    const rejected = await page.evaluate(async () => {
      const { BrowserChatEngine } = await import('./engine.js');
      try { new BrowserChatEngine({ preferredDevice: 'webgpu' }); return ''; }
      catch (e) { return String(e); }
    });
    expect(rejected).toContain('webgpu_not_implemented');
    await context.setOffline(true);
    const requestCount = requests.length;
    const code = String(randomInt(1000, 10000));
    const prompts = [
      `Remember this: my secret code is ${code}. Reply with only the code. /no_think`,
      'What is my secret code plus one? Reply with only the resulting number. /no_think',
      'What was my original secret code? Reply with only that code. /no_think',
    ];
    for (const prompt of prompts) {
      await page.locator('#prompt').fill(prompt);
      await page.locator('#send').click();
      await page.waitForFunction(count => window.qwenTurns.length > count, turns.length, { timeout: 120_000 });
      const result = await page.evaluate(() => window.qwenTurns.at(-1));
      turns.push({ prompt, ...result });
      console.log(JSON.stringify({ turn: turns.length, prompt, answer: result.content, durationMs: result.durationMs }));
      expect(result.device).toBe('cpu');
      expect(result.firstTokenMs).toBeGreaterThan(0);
      expect(result.finishReason).toBe('stop');
    }
    expect(turns[0].content.trim()).toBe(code);
    // Small models may include the equation despite the requested terse format.
    expect(turns[1].content.trim()).toMatch(new RegExp(`\\b${Number(code) + 1}[.!]?$`));
    expect(turns[2].content.trim()).toBe(code);
    expect(turns[2].request.map(m => m.role)).toEqual(['user', 'assistant', 'user', 'assistant', 'user']);
    expect(turns[2].request[0].content).toContain(code);
    expect(turns[2].prompt).not.toContain(code);
    await page.screenshot({ path: testInfo.outputPath('qwen-chat.png'), fullPage: true });
    await page.locator('#reset').click();
    await expect(page.locator('#transcript')).toBeEmpty();
    expect(await page.evaluate(() => qwen.history)).toEqual([]);
    await page.locator('#prompt').fill('What is my secret code? If I have not told you in this conversation, say UNKNOWN. /no_think');
    await page.locator('#send').click();
    await page.waitForFunction(() => window.qwenTurns.length === 4, null, { timeout: 120_000 });
    const control = await page.evaluate(() => window.qwenTurns.at(-1));
    evidence.resetControl = control;
    console.log(JSON.stringify({ resetAnswer: control.content }));
    expect(control.request).toHaveLength(1);
    expect(JSON.stringify(control.request)).not.toContain(code);
    expect(control.content).toContain('UNKNOWN');
    expect(control.content).not.toContain(code);
    const rejectedContext = await page.evaluate(async () => {
      const before = JSON.stringify(qwen.history);
      try { await qwen.chat('x'.repeat(2048)); return {}; }
      catch (error) { return { error: String(error), unchanged: before === JSON.stringify(qwen.history), busy: qwen.busy }; }
    });
    expect(rejectedContext).toEqual({ error: 'Error: demo_context_limit: start a new conversation', unchanged: true, busy: false });
    expect(requests).toHaveLength(requestCount);
    evidence.offlineInference = true;
    expect(errors).toEqual([]);
  } finally {
    const evidencePath = testInfo.outputPath('qwen-evidence.json');
    await writeFile(evidencePath, JSON.stringify(evidence, null, 2));
    await testInfo.attach('qwen-evidence', { path: evidencePath, contentType: 'application/json' });
    await page.evaluate(() => window.qwen?.dispose()).catch(() => {});
  }
});
