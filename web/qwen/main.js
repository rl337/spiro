import { BrowserChatEngine } from './engine.js';
const status = document.querySelector('#status');
const transcript = document.querySelector('#transcript');
const prompt = document.querySelector('#prompt');
const send = document.querySelector('#send');
const reset = document.querySelector('#reset');
const engine = new BrowserChatEngine({ preferredDevice: 'cpu' });
window.qwen = engine;
window.qwenTurns = [];
const controls = busy => { send.disabled = busy; reset.disabled = busy; };
try {
  await engine.load();
  status.textContent = `Ready on CPU · loaded in ${(engine.loadMs / 1000).toFixed(1)}s`;
  document.body.dataset.ready = '1';
  controls(false);
} catch (error) {
  status.textContent = String(error);
  document.body.dataset.error = String(error);
  document.body.dataset.ready = '0';
  console.error(error);
}
document.querySelector('form').addEventListener('submit', async event => {
  event.preventDefault();
  const text = prompt.value;
  if (!text.trim()) return;
  controls(true);
  const priorTranscript = transcript.textContent;
  transcript.textContent += `You: ${text}\nAssistant: `;
  try {
    const result = await engine.chat(text, token => { transcript.textContent += token; });
    window.qwenTurns.push(result);
    transcript.textContent += '\n\n';
    prompt.value = '';
    status.textContent = `CPU · ${(result.durationMs / 1000).toFixed(1)}s`;
  } catch (error) { transcript.textContent = priorTranscript; status.textContent = String(error); }
  finally { controls(false); }
});
reset.addEventListener('click', () => { engine.reset(); transcript.textContent = ''; });
