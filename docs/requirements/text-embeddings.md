# Text embeddings for an in-process host

Normative contract so [weirdness](https://github.com/rl337/weirdness) can embed text inside its own process by calling UIE, instead of an external embeddings HTTP API. Product mandate: [docs/mandate.md](../mandate.md). Wiring rules: [dependency-injection.md](./dependency-injection.md).

Weirdness stores every chunk as an L2-comparable `f32` vector and records the model id on the chunk. Search embeds the query with the current model and scores only chunks that carry that same id. A failed embed fails the write. The host embeds **before** it takes its database lock, in batches of up to 64 strings. Body chunks are at most `CHUNK_MAX_CHARS` (default 1200 characters).

UIE today can run an [inference executor](../stories/007-models-loaders.md) over tensors ([US-009](../stories/009-onnx-mnist.md) ONNX, [US-010](../stories/010-llm-text-completions.md) GGUF completions). Nothing in the [modality catalog](../stories/015-modality-catalog.md) is **text → embedding**. `expect_clip_embeddings` is a diffusion conditioning check, not a sentence embedder. The [byte channel](../stories/020-inference-byte-channel.md) is the wrong primary path for this host: weirdness is a Rust binary and should call a library.

This file is the contract. Shipping story: [US-033](../stories/033-text-embeddings.md) (`crates/uie-embeddings`). Do not treat this document alone as that story.

## Requirements

### EMB-01: In-process library

UIE MUST expose text embedding as a Rust library a host can depend on and call in-process.

The call MUST NOT require a Unix socket, an HTTP server, or `InProcessChannel` framing. A later byte-channel mapping MAY exist; weirdness will not use it for the hot path.

The embedder MUST be `Send + Sync`. Concurrent `embed` calls MUST be safe. An internal lock around a single runnable, as `OnnxModel` does today, is enough.

`embed` MUST NOT call back into the host and MUST NOT read `UIE_*` environment variables ([DI-03](./dependency-injection.md)).

### EMB-02: Strings in, vectors out

The host-facing operation takes an ordered batch of UTF-8 strings and returns one `Vec<f32>` per string, in the same order, all of equal length.

Tokenization, pooling, and normalization happen inside UIE. The host does not supply token ids, attention masks, or a pooling strategy.

The reference pooling policy MUST be the one published for that weight file (CLS or mean). It MUST stay stable for a given model id.

Returned vectors MUST be L2-normalized. Weirdness scores them with cosine similarity, which is then a dot product.

An empty input returns an empty output and MUST NOT run the network.

The operation MUST accept at least 64 inputs per call. It MAY split that batch internally.

### EMB-03: Identity and dimension

Each loaded embedder MUST report:

- `model_id`: stable for those weights. A path, file name, or floating label is not enough if two files could collide. Include a content hash or an explicit revision in the id.
- `dimensions`: the vector length, known after load and identical on every successful embed.

Weirdness stores `model_id` on every chunk. After a model change it re-embeds. A renamed file that still hashes the same MUST keep the same `model_id`. A different tensor MUST NOT reuse an id.

### EMB-04: Context fits a hub chunk

The reference model MUST accept at least 512 tokens.

A 1200-character English chunk is about 300 tokens. A 256-token model would silently drop the tail of a normal hub chunk. Inputs over the window MUST be truncated with a documented rule (which end, and whether a stride is used). Truncation MUST NOT change `dimensions` or `model_id`.

### EMB-05: Failure is total

Any of the following is an error, and no vectors are returned:

- batch length differs from the input length
- a vector is empty or a different length from the others
- the runnable fails
- weights or the tokenizer are missing at load time

UIE MUST NOT substitute a hash embedder or zeros when load or inference fails.

### EMB-06: Pi 5 budget

The required backend is CPU. [US-030](../stories/030-pi-gpu-backend.md) MAY accelerate the same executor later and MUST NOT be required to embed.

The reference weight class is a small English sentence model, 384 dimensions: `BAAI/bge-small-en-v1.5` or `sentence-transformers/all-MiniLM-L6-v2`, quantized if that preserves EMB-02 and EMB-03. Resident size of the loaded model SHOULD stay under 512 MB so a 4 GB Raspberry Pi 5 can run it beside the hub.

Admission MUST go through the existing budget path ([US-025](../stories/025-bounded-model-admission.md)). A model that does not fit the configured budget is rejected at load, not at the first query.

Thread count MUST be injectable and SHOULD default to a small count (1–4) so the hub keeps CPU for SQLite and HTTP.

A batch of 32 strings of 1200 characters SHOULD complete on a Pi 5 without swapping. Latency is not a fixed millisecond gate; it MUST be visible on the existing [execution recorder](../stories/023-inference-telemetry.md).

### EMB-07: Bundle, not a bare graph

A loadable embedder is a directory the [model loader registry](../stories/007-models-loaders.md) can admit ([US-021](../stories/021-model-registry-scan.md)): weights plus the matching tokenizer. A lone `.onnx` graph is not sufficient, because the host does not tokenize.

`embed` MUST NOT download weights. The host or the operator supplies the directory.

Loader selection stays on `ModelLoader` (`format_id`, `can_load`, `load` → `InferenceExecutor`). The string API MAY sit on a thin wrapper over that executor. It MUST still record phases on `ExecutionRecorder`.

### EMB-08: Composition root

The model directory, expected dimension, and thread count MUST be bound in a `uie-iv` module and owned by the injectable ([DI-01](./dependency-injection.md), [DI-05](./dependency-injection.md)).

Library code MUST NOT construct a live embedder as a fallback when the binding is missing ([DI-02](./dependency-injection.md)).

### EMB-09: CI stub, live weights optional

CI MUST run a stub embedder with a fixed `model_id`, a fixed dimension, and deterministic normalized vectors. The acceptance test asserts telemetry (or story events) the same way other executor tests do.

Live weights stay opt-in, same policy as the other live model tests. The stub MUST NOT be described as the reference model id from EMB-06.

## What weirdness will do with this

When the crate exists, weirdness will add an `Embedder` implementation that calls it and will keep the OpenAI-compatible HTTP client as the other backend. `model_id` maps to the hub’s `embedding_model` column. `dimensions` maps to `EMBEDDING_DIM`. Search, reindex, and “no chunk without a vector” stay as they are.

## Out of scope

- Chat or completion. GGUF text generation is [US-010](../stories/010-llm-text-completions.md).
- Image or CLIP embeddings.
- A token-id API as the host surface.
- Running the embedder in a second process beside weirdness.
- Choosing the Pi GPU backend before the CPU path works.
