# US-019: Image-to-image (editing and img2img)

**Status:** planned  
**Personas:** Tracy  
**Crate / path:** future `uie-diffusion` (shared with [US-012](./012-diffusion-text-to-image.md))

## User story

**As** Tracy, who has a rough sketch but wants a polished poster  
**I want** a model that takes an existing image plus guidance and produces a new image  
**So that** UIE covers **image-to-image** and inpainting—not only text-to-image.

## In plain terms

**Image → image** (often **image + text → image**):

1. Start image from file or [US-008](./008-spiro-demo.md) output.
2. Optional text **prompt** from [US-012](./012-diffusion-text-to-image.md).
3. [Diffusion model](../glossary.md#diffusion-model) img2img or inpainting loop writes updated pixels.

Indexed under [US-015](./015-modality-catalog.md); shares backends with US-012.

## Related stories

- [US-012](./012-diffusion-text-to-image.md)
- [US-008](./008-spiro-demo.md)
- [US-015](./015-modality-catalog.md)

## Terms used

- [Diffusion model](../glossary.md#diffusion-model)
- [Text-to-image](../glossary.md#text-to-image)
- [Modality](../glossary.md#modality)

## Acceptance notes

- **Today:** story only.
- **Future acceptance:** img2img CLI with `--strength` flag documented in crate README.