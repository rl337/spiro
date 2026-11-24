import init, { WasmInferenceEngine } from "./pkg/uie_wasm.js";

const statusEl = document.getElementById("status");

function setStatus(text, ok) {
  statusEl.textContent = text;
  statusEl.className = ok === true ? "ok" : ok === false ? "err" : "";
}

async function fetchBytes(url) {
  const res = await fetch(url);
  if (!res.ok) {
    throw new Error(`fetch ${url}: ${res.status}`);
  }
  return new Uint8Array(await res.arrayBuffer());
}

async function main() {
  setStatus("init wasm…");
  await init();

  // Prefer CPU; webgpu must fail loudly if requested.
  const engine = new WasmInferenceEngine("cpu");
  setStatus("fetch model + fixture…");
  const modelBytes = await fetchBytes("./models/mnist-8.onnx");
  const fixtureBytes = await fetchBytes("./models/digit_one_f32.bin");

  engine.loadOnnx("mnist-8", modelBytes);
  const result = engine.inferMnistF32(fixtureBytes);

  const lines = [
    `device: ${result.device}`,
    `model: ${result.model}`,
    `predicted digit: ${result.digit}`,
    `cpu_time_ms: ${result.cpu_time_ms}`,
    `logits: ${JSON.stringify(result.logits)}`,
  ];
  const ok = result.digit === 1 && result.device === "cpu";
  setStatus(lines.join("\n"), ok);

  // Hooks for Playwright.
  document.body.dataset.digit = String(result.digit);
  document.body.dataset.device = String(result.device);
  document.body.dataset.cpuTimeMs = String(result.cpu_time_ms);
  document.body.dataset.ready = ok ? "1" : "0";
}

main().catch((err) => {
  console.error(err);
  setStatus(String(err), false);
  document.body.dataset.ready = "0";
  document.body.dataset.error = String(err);
});
