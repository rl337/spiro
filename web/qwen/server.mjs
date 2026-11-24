// US-026: static assets and a single operator-selected GGUF; no inference endpoint.
import http from 'node:http';
import { createReadStream } from 'node:fs';
import { stat, realpath } from 'node:fs/promises';
import { resolve, dirname, extname, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { homedir } from 'node:os';

const root = dirname(fileURLToPath(import.meta.url));
const model = resolve(process.env.SPIRO_QWEN_MODEL ?? resolve(homedir(), '.lmstudio/models/Qwen/Qwen3-0.6B-GGUF/Qwen3-0.6B-Q8_0.gguf'));
const types = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.wasm': 'application/wasm' };

http.createServer(async (req, res) => {
  res.setHeader('Cross-Origin-Opener-Policy', 'same-origin');
  res.setHeader('Cross-Origin-Embedder-Policy', 'require-corp');
  res.setHeader('Cross-Origin-Resource-Policy', 'same-origin');
  if (!['GET', 'HEAD'].includes(req.method)) {
    res.writeHead(405).end();
    return;
  }
  try {
    const path = decodeURIComponent(new URL(req.url, 'http://localhost').pathname);
    const isModel = path === '/model.gguf';
    const file = await realpath(isModel ? model : resolve(root, `.${path === '/' ? '/index.html' : path}`));
    if (!isModel && !file.startsWith(root + sep)) {
      res.writeHead(403).end();
      return;
    }
    const info = await stat(file);
    if (!info.isFile()) throw new Error('not a file');
    res.setHeader('Content-Type', types[extname(file)] ?? 'application/octet-stream');
    res.setHeader('Accept-Ranges', 'bytes');
    let start = 0, end = info.size - 1;
    if (req.headers.range) {
      const match = /^bytes=(\d+)-(\d*)$/.exec(req.headers.range);
      if (!match) { res.writeHead(416).end(); return; }
      start = Number(match[1]);
      end = match[2] ? Number(match[2]) : end;
      if (start > end || end >= info.size) { res.writeHead(416).end(); return; }
      res.setHeader('Content-Range', `bytes ${start}-${end}/${info.size}`);
      res.statusCode = 206;
    }
    res.setHeader('Content-Length', end - start + 1);
    if (req.method === 'HEAD') { res.end(); return; }
    const stream = createReadStream(file, { start, end });
    stream.on('error', () => res.destroy());
    res.on('close', () => stream.destroy());
    stream.pipe(res);
  } catch {
    res.writeHead(404).end('File unavailable');
  }
}).listen(Number(process.env.PORT ?? 18766), '127.0.0.1', () => {
  console.log('Qwen browser demo: http://127.0.0.1:' + (process.env.PORT ?? 18766));
});
