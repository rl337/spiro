# US-023: Inference execution telemetry and traces

**Status:** shipped (phase device tags + CPU/GPU time; GGUF GPU when env set)  
**Personas:** Pete, Jack  
**Crate / path:** `crates/uie-telemetry`, `uie-core::infer_with_trace`, IPC `trace` field

## User story

**As** Pete, comparing quantization and batching tricks—and **as** Jack, shipping loaders  
**I want** every inference run to emit a structured **execution trace** (TTFT, throughput, CPU vs GPU time)  
**So that** Spiro can benchmark optimization ideas from the wild on equal footing.

## In plain terms

1. [`ExecutionRecorder`](../crates/uie-telemetry) times phases (`executor_infer`, `prefill`, `decode`, `ipc_hop`, …) and tags [`ComputeDevice`](../crates/uie-telemetry) (`cpu`, `gpu`, …).
2. [`InferMetrics`](../crates/uie-telemetry) holds **TTFT**, **prefill/decode ms**, **tokens/sec**, and summed **CPU/GPU** phase time.
3. **`InferenceExecutor::infer` must take `&mut ExecutionRecorder`** — stubs record `stub_modality_*`, `stub_validate_inputs`, `stub_policy_infer`, fill TTFT-style **metrics**, tag **`user_story`**, and emit `uie_stub` tracing events; acceptance tests assert those fields.
4. `InferenceRuntime::infer` always finishes a trace (see [`InferOutcome`](../crates/uie-core/src/infer.rs)).
4. GGUF completion fills metrics from real generation ([US-010](./010-llm-text-completions.md)).
5. IPC [`InferResult`](./020-inference-byte-channel.md) includes optional `trace` summary for clients.
6. Set `UIE_TRACE_JSONL=/path/traces.jsonl` to append full JSON traces for offline analysis (Twitter thread spreadsheet fuel).
7. Native loaders tag phases with [`ComputeDevice`](../../crates/uie-telemetry): ONNX is CPU-only; GGUF sets GPU metrics only when `UIE_GGUF_GPU=1`.

## Related stories

- [US-020](./020-inference-byte-channel.md) — trace on wire
- [US-010](./010-llm-text-completions.md) — TTFT on LLM
- [US-022](./022-byte-channel-pipeline.md) — per-hop hop_ms logs

## Terms used

- [Inference runtime](../glossary.md#inference-runtime)
- [Byte channel](../glossary.md#byte-channel)

## Acceptance notes

- `cargo test -p uie-acceptance --test us_023_telemetry`
- `UIE_TRACE_JSONL=./traces.jsonl cargo run -p uie-llm -- ...` appends one JSON line per completion

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_023_telemetry.rs` | `cargo test -p uie-acceptance --test us_023_telemetry` |

## Observability

- **Trace:** `assert_phases`, `assert_stub_executor_telemetry`, `TraceSummary` JSON for IPC
- **CI test:** `us_023_telemetry.rs`
