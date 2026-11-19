# US-012: Diffusion models (text-to-image)

**Status:** planned  
**Personas:** Tracy, Jenna  
**Crate / path:** future `uie-diffusion` (not shipped)

## User story

**As** Tracy, who needs splashy visuals—and **as** Jenna, who judges the final frame  
**I want** a **diffusion** model that turns a text description into an image  
**So that** UIE can do **text-to-image** like the Spiro demo produces a JPEG—but from a prompt, not math curves.

## In plain terms

**Text → image** generation:

1. User supplies a text **prompt** (and optional negative prompt).
2. UIE encodes text to conditioning [tensors](./002-tensors.md).
3. A **[diffusion model](../glossary.md#diffusion-model)** denoises latent noise over many steps.
4. Output is an image [tensor](./002-tensors.md) (for example `U8` or `F32` RGB), saved like [US-008](./008-spiro-demo.md).

Unlike Spiro, this is a heavy [workload](../glossary.md#workload); [Pete](./003-backends-cpu.md) expects a GPU [backend](../glossary.md#backend) for practical latency.

Related variants live in [US-015](./015-modality-catalog.md) (image-to-image, inpainting).

## Related stories

- [US-008](./008-spiro-demo.md) — today’s image output path
- [US-002](./002-tensors.md) — image tensors
- [US-019](./019-image-to-image.md) — img2img / editing
- [US-015](./015-modality-catalog.md)

## Terms used

- [Diffusion model](../glossary.md#diffusion-model)
- [Text-to-image](../glossary.md#text-to-image)
- [Prompt](../glossary.md#prompt)
- [Modality](../glossary.md#modality)

## Acceptance notes

- **Today:** story only; no diffusion loader.
- **Future acceptance:** `cargo run -p uie-diffusion -- --prompt "sunset"` writes `out.png`.
- **Limitation:** step schedulers and VAE decode are format-specific; likely separate loader crate.