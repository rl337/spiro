# US-022: Multi-model pipelines via byte channel

**Status:** shipped (in-process multi-hop CI; socket per-hop optional)  
**Personas:** Tracy, Pete  
**Crate / path:** `crates/uie-pipeline`

## User story

**As** Tracy, running a Comfy-style stack—and **as** Pete, avoiding bespoke glue  
**I want** several [inference executors](../glossary.md#inference-executor) chained through the same [byte channel](../glossary.md#byte-channel) framing  
**So that** we can prove inter-model flows before faster **inter-model conduits** exist.

## In plain terms

1. `ByteChannelPipelineRunner` runs each stage as an in-process `infer` IPC message.
2. Output tensors of hop *N* become inputs of hop *N+1* (`TensorPayload` on the wire).
3. `build_comfy_byte_pipeline` orders text encoder → diffusion → VAE from [US-021](./021-model-registry-scan.md) scan, with stub fallbacks for missing files.
4. If byte-channel latency is too high later, add a direct tensor conduit; keep acceptance tests on the byte path.

## Related stories

- [US-020](./020-inference-byte-channel.md)
- [US-021](./021-model-registry-scan.md)
- [US-012](./012-diffusion-text-to-image.md)

## Terms used

- [Byte channel](../glossary.md#byte-channel)
- [Inference executor](../glossary.md#inference-executor)

## Acceptance notes

- `cargo test -p uie-pipeline`
- `cargo test -p uie-acceptance --test us_022_byte_pipeline`

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_022_byte_pipeline.rs` | `cargo test -p uie-acceptance --test us_022_byte_pipeline` |
| Rust | `crates/uie-pipeline/tests/byte_stack.rs` | `cargo test -p uie-pipeline` |
| Rust (live) | `crates/uie-acceptance/tests/live_models.rs` | `UIE_ACCEPTANCE_MODE=live cargo test -p uie-acceptance --test live_models live_us_022` |

## Observability

- **Trace:** per-hop `assert_ipc_infer_trace` on pipeline byte channel
- **CI:** `us_022_byte_pipeline.rs`, `uie-pipeline/tests/byte_stack.rs`
- **Live:** `live_us_022_byte_pipeline_workspace_onnx_or_stub` (infer trace when run)
