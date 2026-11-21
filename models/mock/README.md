# Mock model roots (test detector overlay)

Stub weights are laid out like **LM Studio** and **ComfyUI** trees so
[`TestModelDetector`](../../crates/uie-catalog/src/detector.rs) can discover them the same way as real GGUF / safetensors.

| Root id | Layout | Example id |
|---------|--------|------------|
| `mock-lmstudio` | `publisher/repo/*.stub` | `mock-lmstudio:MockUIE/qwen3-2b-mock/llm.stub` |
| `mock-comfy` | `checkpoints/`, `text_encoders/`, … | `mock-comfy:checkpoints/mock_diffusion.stub` |

Manifest bodies are copies of [`../stubs/`](../stubs/) (one-line modality id).

Enable overlay:

```rust
use uie_catalog::{ModelCatalog, TestModelDetector, DirectoryModelDetector};
let catalog = ModelCatalog::from_roots_with_detector(
    roots,
    &TestModelDetector::workspace_mock(repo_root),
);
```

Or: `cargo run -p uie-e2e -- list-models` / `run mock_registry_discovery`.