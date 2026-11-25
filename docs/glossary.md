# Glossary

Alphabetical list of terms used in [user stories](./stories/README.md). Anchors match link targets (`#tensor`, etc.).

---

### Capability probe {#capability-probe}

A **capability probe** detects CPU/GPU features (e.g. NEON, dot-product) before selecting an optimized kernel path, so missing features fall back or error explicitly instead of trapping ([US-029](./stories/029-capability-aware-kernels.md)).

**See also:** [US-003](./stories/003-backends-cpu.md), [US-029](./stories/029-capability-aware-kernels.md)

---

### Chatbot {#chatbot}

A **chatbot** is an application that keeps conversation state and repeatedly asks a [large language model](#large-language-model) for the next [completion](#completion). In UIE, Liz’s target design binds `prompt` and history in a [session](#session) ([US-010](./stories/010-llm-text-completions.md)).

**See also:** [US-010](./stories/010-llm-text-completions.md)

---

### Completion {#completion}

A **completion** is the model-generated text that continues a [prompt](#prompt)—one assistant turn in a [chatbot](#chatbot). Completions are **text → text** [modality](#modality) output.

**See also:** [US-010](./stories/010-llm-text-completions.md)

---

### Composition root {#composition-root}

A **composition root** wires CI vs live dependencies through [`uie-iv`](../crates/uie-iv/README.md) modules (Guice / [pyiv](https://github.com/rl337/pyiv) style), instead of `if live` at each call site.

**See also:** [US-024](./stories/024-dependency-injection.md)

---

### Execution trace {#execution-trace}

An **execution trace** is a structured record of one inference run: phase timings, [`InferMetrics`](#infer-metrics) (TTFT, throughput), and CPU vs GPU time. Emitted by [`uie-telemetry`](../crates/uie-telemetry) and optional `UIE_TRACE_JSONL`.

**See also:** [US-023](./stories/023-inference-telemetry.md)

---

### Inference state {#inference-state}

**Inference state** is the per-session working memory of a running model—typically KV cache, scratch, and related buffers—distinct from weight residency. Bounded by [US-027](./stories/027-bounded-inference-state.md).

**See also:** [US-004](./stories/004-runtime-sessions.md), [US-010](./stories/010-llm-text-completions.md), [US-027](./stories/027-bounded-inference-state.md)

---

### Infer metrics {#infer-metrics}

**Infer metrics** are aggregate numbers for a run: prompt/completion token counts, **time to first token (TTFT)**, prefill vs decode latency, tokens/sec, and summed time on CPU vs GPU. Used to evaluate quantization, batching, and kernel swaps.

**See also:** [US-023](./stories/023-inference-telemetry.md)

---

### Byte channel {#byte-channel}

A **byte channel** is UIE’s transport for inference: clients and the engine exchange **length-prefixed JSON** frames carrying `infer` requests and tensor payloads. Implementations include in-process [`InProcessChannel`](../crates/uie-ipc) and Unix domain sockets via [`uie-serve`](../crates/uie-serve).

**See also:** [US-020](./stories/020-inference-byte-channel.md)

---

### Benchmark bracket {#benchmark-bracket}

A **benchmark bracket** is a controlled **control / candidate / control** experiment pair used to decide whether an optimization is an eligible win, drift/inconclusive, or invalid ([US-028](./stories/028-constrained-benchmark-controls.md)).

**See also:** [US-023](./stories/023-inference-telemetry.md), [US-028](./stories/028-constrained-benchmark-controls.md)

---

### Backend {#backend}

A **backend** is the piece that actually runs compute: it allocates memory and (eventually) executes individual operations. UIE talks to backends through a shared interface so the same [runtime](#inference-runtime) can target CPU or accelerators later.

**See also:** [US-003](./stories/003-backends-cpu.md)

---

### CpuBackend {#cpubackend}

The **CPU backend** is the reference [backend](#backend) that runs on the host machine. It is always available and implements a small set of [operations](#operation) today (for example filling a [tensor](#tensor) with a constant).

**See also:** [US-003](./stories/003-backends-cpu.md)

---

### NPU {#npu}

An **NPU** (neural processing unit) is an on-device accelerator specialized for neural-net inference (distinct from a general GPU). UIE plans an NPU [backend](#backend) when hardware/SDK access exists ([US-031](./stories/031-npu-backend.md)).

**See also:** [US-030](./stories/030-pi-gpu-backend.md), [US-031](./stories/031-npu-backend.md), [mandate](./mandate.md)

---

### Diffusion model {#diffusion-model}

A **diffusion model** generates or edits images by gradually denoising latent noise, usually conditioned on text. UIE stories use it for [text-to-image](#text-to-image) and image-to-image ([US-012](./stories/012-diffusion-text-to-image.md), [US-019](./stories/019-image-to-image.md)).

**See also:** [US-012](./stories/012-diffusion-text-to-image.md), [US-019](./stories/019-image-to-image.md)

---

### DataType {#datatype}

A **data type** says how each element in a [tensor](#tensor) is stored: for example 32-bit floats (`F32`) or 8-bit bytes (`U8`). The type fixes the size of each element so shapes can be validated.

**See also:** [US-002](./stories/002-tensors.md)

---

### Device {#device}

A **device** describes *where* a [backend](#backend) runs (host CPU vs a future GPU slot). Today UIE mostly uses the host CPU [device](#device).

**See also:** [US-003](./stories/003-backends-cpu.md)

---

### Execution plan {#execution-plan}

An **execution plan** is an ordered list of [operations](#operation) the engine will run. Plan-based [models](#inference-model) expose a plan instead of a closed black box.

**See also:** [US-005](./stories/005-execution-plans.md)

---

### Graph {#graph}

A **graph** is a named bundle of inputs, outputs, and [operations](#operation). It can be turned into an [execution plan](#execution-plan) for the [runtime](#inference-runtime).

**See also:** [US-005](./stories/005-execution-plans.md), [US-006](./stories/006-operations.md)

---

### Image-to-text {#image-to-text}

**Image-to-text** is the [modality](#modality) pair where an image [tensor](#tensor) goes in and descriptive text comes out (captioning, VQA). See [US-016](./stories/016-image-to-text.md).

**See also:** [US-016](./stories/016-image-to-text.md), [US-017](./stories/017-multimodal-text-image-to-text.md)

---

### Inference executor {#inference-executor}

An **inference executor** is a loaded [model](#inference-model) you can run end-to-end: hand it input [tensors](#tensor) and receive outputs (for example an ONNX network). This is the path used by [model loaders](#model-loader).

**See also:** [US-007](./stories/007-models-loaders.md), [US-009](./stories/009-onnx-mnist.md)

---

### Judge model {#judge-model}

A **judge model** scores, classifies, or critiques another model’s output—often **text → text** or **text → labels**. Used with generators in [US-011](./stories/011-judge-system-one-models.md).

**See also:** [US-011](./stories/011-judge-system-one-models.md)

---

### Inference model {#inference-model}

An **inference model** (plan-style) exposes metadata plus an [execution plan](#execution-plan) instead of a single `infer` call. Scaffold code still uses this for graph-first workflows.

**See also:** [US-007](./stories/007-models-loaders.md)

---

### Inference runtime {#inference-runtime}

The **inference runtime** is the front door: it owns the active [backend](#backend), registers [model loaders](#model-loader), creates [sessions](#session), and runs plans or direct inference.

**See also:** [US-004](./stories/004-runtime-sessions.md)

---

### Large language model {#large-language-model}

A **large language model** (LLM) predicts language from context—typically from [token](#token) sequences—and powers [completions](#completion) and [chatbots](#chatbot) ([US-010](./stories/010-llm-text-completions.md)).

**See also:** [US-010](./stories/010-llm-text-completions.md)

---

### Io spec {#io-spec}

An **I/O spec** names a model input or output: [tensor](#tensor) [shape](#shape), [data type](#datatype), and a label. Loaders use specs so callers know what to pass in.

**See also:** [US-007](./stories/007-models-loaders.md)

---

### Memory budget {#memory-budget}

A **memory budget** is a configured capacity (often cgroup `memory.max` live, or a fake provider in CI) used to admit or reject model loads and to bound residency in the host cache ([US-025](./stories/025-bounded-model-admission.md)).

**See also:** [US-025](./stories/025-bounded-model-admission.md), [US-024](./stories/024-dependency-injection.md)

---

### Model loader {#model-loader}

A **model loader** knows how to read a file format (for example `.onnx`) and produce an [inference executor](#inference-executor). The [runtime](#inference-runtime) picks the first loader that recognizes a path.

**See also:** [US-007](./stories/007-models-loaders.md), [US-009](./stories/009-onnx-mnist.md)

---

### Modality {#modality}

A **modality** is the kind of data at a model boundary: text, image, or audio (more later). Pairs like text → image are catalogued in [US-015](./stories/015-modality-catalog.md).

**See also:** [US-015](./stories/015-modality-catalog.md)

---

### Multimodal {#multimodal}

**Multimodal** models accept more than one [modality](#modality) per call—for example text plus image → text ([US-017](./stories/017-multimodal-text-image-to-text.md)).

**See also:** [US-017](./stories/017-multimodal-text-image-to-text.md)

---

### Model registry {#model-registry}

The **model registry** is built at runtime by scanning [**model roots**](#model-root) (directories only). Each indexed file gets an id, format, and role (LLM GGUF, Comfy checkpoint, etc.). See [US-021](./stories/021-model-registry-scan.md).

**See also:** [US-021](./stories/021-model-registry-scan.md)

---

### Model root {#model-root}

A **model root** is a configured directory (LM Studio tree, Comfy `models/`, or generic) listed in `model_roots.yaml` without naming individual files.

**See also:** [US-021](./stories/021-model-registry-scan.md)

---

### Model loader registry {#model-loader-registry}

The **model loader registry** holds every registered loader. When you ask the [runtime](#inference-runtime) to open a file, it walks the registry until one loader claims the path.

**See also:** [US-007](./stories/007-models-loaders.md)

---

### Operation {#operation}

An **operation** (op) is one step in a compute [graph](#graph): fill a buffer, apply a map, or a custom kernel. [Backends](#backend) implement ops; the CPU backend supports a minimal set while the graph executor matures.

**See also:** [US-006](./stories/006-operations.md)

---

### Prompt {#prompt}

A **prompt** is the input text (or [token](#token) sequence) you give an LLM or diffusion conditioner before generation. Chatbots append history to the prompt each turn.

**See also:** [US-010](./stories/010-llm-text-completions.md), [US-012](./stories/012-diffusion-text-to-image.md)

---

### Session {#session}

A **session** is a workspace for one run: named [tensors](#tensor) bound to strings (like `"canvas"` or `"input"`). The [runtime](#inference-runtime) can read and write those bindings as ops or models execute.

**See also:** [US-004](./stories/004-runtime-sessions.md)

---

### Speech-to-text {#speech-to-text}

**Speech-to-text** (audio-to-text, ASR) turns audio [tensors](#tensor) into transcripts. See [US-013](./stories/013-speech-to-text.md).

**See also:** [US-013](./stories/013-speech-to-text.md)

---

### Shape {#shape}

A **shape** is the list of dimensions for a [tensor](#tensor), such as `[2048, 2048, 3]` for an RGB image or `[1, 1, 28, 28]` for MNIST. Rank is the number of dimensions; element count is their product.

**See also:** [US-002](./stories/002-tensors.md)

---

### System-one model {#system-one-model}

A **system-one model** is the deliberate “second pass” in a two-stage setup—often a [judge model](#judge-model) that reviews a fast draft ([US-011](./stories/011-judge-system-one-models.md)).

**See also:** [US-011](./stories/011-judge-system-one-models.md)

---

### Text embedding {#text-embedding}

A **text embedding** maps a UTF-8 string to a fixed-length L2-normalized `f32` vector for similarity search. UIE exposes this as an in-process library ([US-033](./stories/033-text-embeddings.md)) for hosts such as weirdness; CI uses a deterministic stub, not a hash-fallback on load failure.

**See also:** [US-033](./stories/033-text-embeddings.md), [text-embeddings.md](./requirements/text-embeddings.md)

---

### Text-to-image {#text-to-image}

**Text-to-image** maps a text [prompt](#prompt) to an image [tensor](#tensor), usually via a [diffusion model](#diffusion-model) ([US-012](./stories/012-diffusion-text-to-image.md)).

**See also:** [US-012](./stories/012-diffusion-text-to-image.md)

---

### Text-to-speech {#text-to-speech}

**Text-to-speech** (text-to-audio) synthesizes waveform or mel outputs from written text ([US-014](./stories/014-text-to-speech.md)).

**See also:** [US-014](./stories/014-text-to-speech.md)

---

### Tensor {#tensor}

A **tensor** is typed, shaped data: bytes in memory plus [shape](#shape) and [data type](#datatype). Everything you feed into UIE—or save from a demo—flows through tensors.

**See also:** [US-002](./stories/002-tensors.md)

---

### Token {#token}

A **token** is a chunk of text the LLM reads or writes internally. Prompts and [completions](#completion) are usually tokenized before [inference](#inference-runtime).

**See also:** [US-010](./stories/010-llm-text-completions.md)

---

### Composition root (uie-iv)

A **composition root** binds CI vs live implementations through [`uie-iv`](../crates/uie-iv/README.md) modules (analogous to [pyiv](https://github.com/rl337/pyiv)), replacing scattered environment conditionals.

**See also:** [US-024](./stories/024-dependency-injection.md)

## UIE {#uie}

**UIE** (Universal Inference Engine) is the Rust workspace in this repo: shared core (`uie-core`), demos (`uie-spiro`, `uie-mnist`), loaders, IPC/serve, and WASM surfaces. The [mandate](./mandate.md) is good inference **at any scale**; when designs must compromise, prefer **edge and embedded** performance.

**See also:** [US-001](./stories/001-why-uie.md), [mandate](./mandate.md)

---

### Edge compute {#edge-compute}

**Edge compute** here means inference on resource-constrained hosts close to the user or device—e.g. Raspberry Pi–class ARM boards, containers with tight cgroup limits, and **embedded WASM** in the browser. It is not a separate product track: it is the **compromise direction** when any-scale designs must choose (see [mandate](./mandate.md)).

**See also:** [US-025](./stories/025-bounded-model-admission.md), [US-026](./stories/026-browser-wasm-inference.md), [US-028](./stories/028-constrained-benchmark-controls.md)

---

### Workload {#workload}

A **workload** is any program UIE runs: a classic demo like the Spirograph, or a neural [model](#inference-model). Workloads help prove the engine pieces fit together.

**See also:** [US-008](./stories/008-spiro-demo.md)