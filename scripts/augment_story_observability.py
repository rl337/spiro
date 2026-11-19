#!/usr/bin/env python3
"""Insert or replace ## Observability sections in docs/stories/*.md from governance copy."""

from __future__ import annotations

from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
STORIES = REPO / "docs" / "stories"

# Per US-0NN observability documentation (CI-enforced header).
COPY: dict[str, str] = {
    "US-001": """## Observability

- **Events (`StoryEventLog`):** `uie_core_present`, `story_doc_present`
- **CI test:** `us_001_why_uie.rs`
""",
    "US-002": """## Observability

- **Events:** `tensor_f32_roundtrip`, `dtype_preserved`
- **CI test:** `us_002_tensors.rs`
""",
    "US-003": """## Observability

- **Events:** `cpu_backend_constructed`, `tensor_allocated_on_cpu`
- **CI test:** `us_003_cpu_backend.rs`
""",
    "US-004": """## Observability

- **Events:** `session_bind`, `runtime_infer_trace`
- **Trace:** `assert_infer_outcome_trace` on stub LLM infer (`runtime_session_bind` + executor phases)
- **CI test:** `us_004_runtime_session.rs`
""",
    "US-005": """## Observability

- **Events:** `execution_plan_emitted`
- **CI test:** `us_005_execution_plan.rs`
""",
    "US-006": """## Observability

- **Events:** `op_kind_fill_constructed`
- **CI test:** `us_006_operations.rs`
""",
    "US-007": """## Observability

- **Events:** `stub_executor_loaded`, `loader_infer_trace`
- **Trace:** `assert_stub_executor_telemetry`, `assert_infer_outcome_trace`
- **CI test:** `us_007_model_loader.rs`
- **Live:** `live_us_007_load_path_from_env` records load/infer events when `UIE_LIVE_MODEL_PATH` is set
""",
    "US-008": """## Observability

- **Events:** `spiro_demo_entrypoint`, `spiro_module_sources_present`
- **CI test:** `us_008_spiro.rs`
""",
    "US-009": """## Observability

- **Events:** `mnist_fixture_present`, `mnist_input_tensor_bytes`
- **CI test:** `us_009_mnist.rs`
- **Live:** `live_us_009_mnist_onnx_infer` uses `assert_infer_outcome_trace` (ONNX phases)
""",
    "US-010": """## Observability

- **Trace:** `assert_stub_executor_telemetry` (`stub_validate_inputs`, `stub_policy_infer`, `runtime_session_bind`)
- **CI test:** `us_010_019_modalities.rs` → `us_010_llm_stub`
- **Live:** `live_us_010_gguf_text_completion` asserts `CompleteResult.metrics`
""",
    "US-011": """## Observability

- **Trace:** stub executor phases via `us_011_judge_stub`
- **CI test:** `us_010_019_modalities.rs`
""",
    "US-012": """## Observability

- **Trace:** stub executor phases via `us_012_diffusion_stub`
- **CI test:** `us_010_019_modalities.rs`
""",
    "US-013": """## Observability

- **Trace:** stub executor phases via `us_013_stt_stub`
- **CI test:** `us_010_019_modalities.rs`
""",
    "US-014": """## Observability

- **Trace:** stub executor phases via `us_014_tts_stub`
- **CI test:** `us_010_019_modalities.rs`
""",
    "US-015": """## Observability

- **Events:** `modality_catalog_complete`, per-modality `modality_indexed:*`
- **CI test:** `us_015_modality_catalog.rs`
""",
    "US-016": """## Observability

- **Trace:** stub executor phases via `us_016_image_to_text_stub`
- **CI test:** `us_010_019_modalities.rs`
""",
    "US-017": """## Observability

- **Trace:** stub executor phases via `us_017_vlm_stub`
- **CI test:** `us_010_019_modalities.rs`
""",
    "US-018": """## Observability

- **Trace:** stub executor phases via `us_018_audio_to_audio_stub`
- **CI test:** `us_010_019_modalities.rs`
""",
    "US-019": """## Observability

- **Trace:** stub executor phases via `us_019_image_to_image_stub`
- **CI test:** `us_010_019_modalities.rs`
""",
    "US-020": """## Observability

- **Trace:** `assert_ipc_infer_trace` / `assert_ipc_infer_trace_summary` on `InferResult.trace`
- **Python:** `assert_ipc_infer_has_trace`
- **CI:** `us_020_byte_channel.rs`, `uie-ipc/tests/roundtrip.rs`, `test_us_020_ipc.py`
""",
    "US-021": """## Observability

- **Events:** `registry_scan_entry`, `registry_entry_id_prefix`; crate test adds `registry_scan_multi_layout`
- **CI:** `us_021_model_registry.rs`, `uie-catalog/tests/scan.rs`
- **Live:** `live_us_021_*` records `registry_live_scan` events (skipped in CI)
""",
    "US-022": """## Observability

- **Trace:** per-hop `assert_ipc_infer_trace` on pipeline byte channel
- **CI:** `us_022_byte_pipeline.rs`, `uie-pipeline/tests/byte_stack.rs`
- **Live:** `live_us_022_byte_pipeline_workspace_onnx_or_stub` (infer trace when run)
""",
    "US-023": """## Observability

- **Trace:** `assert_phases`, `assert_stub_executor_telemetry`, `TraceSummary` JSON for IPC
- **CI test:** `us_023_telemetry.rs`
""",
}


def story_doc(story_id: str) -> Path | None:
    n = story_id.split("-")[1]
    for p in STORIES.glob(f"{n}-*.md"):
        return p
    return None


def upsert_observability(path: Path, body: str) -> None:
    text = path.read_text(encoding="utf-8")
    marker = "## Observability"
    if marker in text:
        before, _, after = text.partition(marker)
        # drop old section up to next ## or EOF
        rest = after
        if "\n## " in rest:
            rest = rest.split("\n## ", 1)[1]
            rest = "## " + rest
        else:
            rest = ""
        new_text = before.rstrip() + "\n\n" + body.strip() + "\n"
        if rest.strip():
            new_text += "\n" + rest.lstrip()
    else:
        new_text = text.rstrip() + "\n\n" + body.strip() + "\n"
    path.write_text(new_text, encoding="utf-8")


def main() -> None:
    for sid, section in COPY.items():
        doc = story_doc(sid)
        if not doc:
            raise SystemExit(f"missing story doc for {sid}")
        upsert_observability(doc, section)
    print(f"augmented {len(COPY)} story Observability sections")


if __name__ == "__main__":
    main()