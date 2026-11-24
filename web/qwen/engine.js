import { Wllama } from './node_modules/@wllama/wllama/esm/index.js';

// US-026: browser GGUF executor. Tract's Rust ONNX executor remains separate.
export class BrowserChatEngine {
  #runtime;
  #messages = [];
  #busy = false;
  #loaded = false;
  #closed = false;
  constructor({ preferredDevice = 'auto' } = {}) {
    if (!['auto', 'cpu'].includes(preferredDevice)) {
      throw new Error(preferredDevice === 'webgpu' ? 'webgpu_not_implemented' : 'unknown preferredDevice');
    }
    this.device = 'cpu';
    this.settings = Object.freeze({ n_ctx: 2048, n_threads: 4, n_gpu_layers: 0, seed: 42, n_parallel: 1, offload_kqv: false });
    this.generation = Object.freeze({ temperature: 0, seed: 42, max_tokens: 96, cache_prompt: false, chat_template_kwargs: { enable_thinking: false } });
    this.#runtime = new Wllama({ default: new URL('./node_modules/@wllama/wllama/esm/wasm/wllama.wasm', import.meta.url).href });
  }
  get history() { return structuredClone(this.#messages); }
  get busy() { return this.#busy; }
  async load() {
    if (this.#closed || this.#loaded || this.#busy) throw new Error('engine_already_loaded_or_busy');
    this.#busy = true;
    try {
      const start = performance.now();
      await this.#runtime.loadModelFromUrl(new URL('./model.gguf', import.meta.url).href, this.settings);
      this.info = this.#runtime.getLoadedContextInfo();
      this.multithread = this.#runtime.isMultithread();
      this.loadMs = performance.now() - start;
      this.#loaded = true;
    } finally { this.#busy = false; }
  }
  async chat(text, onToken = () => {}) {
    if (!this.#loaded || this.#busy) throw new Error('engine_not_ready_or_busy');
    if (!text.trim()) throw new Error('empty_message');
    const request = [...this.#messages, { role: 'user', content: text }];
    // Bound this small demo; a production executor should count template tokens.
    if (request.length > 15 || new TextEncoder().encode(JSON.stringify(request)).length > 1024) {
      throw new Error('demo_context_limit: start a new conversation');
    }
    this.#busy = true;
    const start = performance.now();
    let content = '', firstTokenMs = null, finishReason = null;
    try {
      await this.#runtime.createChatCompletion({
        ...this.generation, messages: request, stream: true,
        onData: chunk => {
          const choice = chunk.choices?.[0];
          const token = choice?.delta?.content ?? '';
          if (token) {
            firstTokenMs ??= performance.now() - start;
            content += token;
            onToken(token);
          }
          if (choice?.finish_reason) finishReason = choice.finish_reason;
        },
      });
      this.#messages = [...request, { role: 'assistant', content }];
      return { content, request: structuredClone(request), finishReason, firstTokenMs, durationMs: performance.now() - start, device: this.device };
    } finally { this.#busy = false; }
  }
  reset() {
    if (this.#busy) throw new Error('engine_busy');
    // Requests replay explicit history with cache_prompt:false; no prior slot cache is reused.
    this.#messages = [];
  }
  async dispose() {
    if (this.#busy) throw new Error('engine_busy');
    if (this.#closed) return;
    this.#busy = true;
    this.#closed = true;
    this.#loaded = false;
    try { await this.#runtime.exit(); }
    finally { this.#messages = []; this.#busy = false; }
  }
}
