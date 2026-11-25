# US-032: Image filter stub (JSON + image → image)

**Status:** partial (CPU stub; Pi V3D GPU path later)  
**Personas:** Tracy, Pete  
**Crate / path:** `crates/uie-stub-models` (`ModalityKind::ImageFilter`), `models/stubs/image_filter.stub`

## User story

**As a** Tracy  
**I want** an image processor that looks like an image→image model stub  
**So that** pipelines can submit a structured JSON filter list plus pixels and get a filtered image back—before a Pi GPU or real diffusion img2img exists.

## In plain terms

1. Inputs mirror a conditional image model: **`pixel_values`** (NCHW f32) + **`filter_json`** (UTF-8 JSON in a U8 tensor).
2. JSON names a short filter chain (invert, grayscale, brightness, contrast).
3. Output is **`sample`** / filtered NCHW image—same shape contract as [US-019](./019-image-to-image.md).
4. CI uses a CPU stub that actually applies filters (not blank noise).
5. Pi V3D GPU kernels are a later enablement POC under [US-030](./030-pi-gpu-backend.md); not required for this story’s CI.

## Related stories

- [US-019](./019-image-to-image.md) — diffusion-style img2img (image only today)
- [US-012](./012-diffusion-text-to-image.md) — text→image sibling shape
- [US-030](./030-pi-gpu-backend.md) — optional V3D acceleration later
- [US-015](./015-modality-catalog.md) — modality index

## Terms used

- [Tensor](../glossary.md#tensor)
- [Stub](../glossary.md#stub) (if present)
- [Edge compute](../glossary.md#edge-compute)

## Acceptance notes

- **Today (CI):** `image_filter.stub` + `apply_filters_nchw` from JSON; telemetry via stub executor phases.
- **Example JSON:** `{"filters":[{"op":"invert"},{"op":"brightness","amount":0.1}]}`
- **Live / Pi GPU:** optional; CPU remains the default for CI.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_032_image_filter.rs` | `cargo test -p uie-acceptance --test us_032_image_filter` |

## Observability

- **Trace:** `assert_stub_modality_telemetry` (`stub_modality_image_filter`, validate + policy phases).
- **CI test:** `us_032_image_filter.rs`
