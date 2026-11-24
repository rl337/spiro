# Inference-optimization inspiration

Curated from the [rl337/trouble](https://github.com/rl337/trouble) ingest on November 24, 2026 (America/Los_Angeles). Summaries are paraphrased from each post's public Trouble record only. Source ids and scan counts are in [`sources/2026-11-24-inference-optimizations.json`](./sources/2026-11-24-inference-optimizations.json).

UIE context used when judging relevance: `uie-core` already has tensors, `CpuBackend`, `InferenceRuntime` / `Session`, and an `Op` graph, but the graph executor is not wired yet. `uie-loader-gguf` / `uie-llm` run GGUF text completions. `uie-serve` / `uie-ipc` provide Unix-socket serving. README next steps are real loaders (ONNX / SafeTensors / GGUF), the graph executor, backend plugins, and a Python bridge.

## Memory, offload, and MoE routing

### Stream MoE experts from SSD (InferSSD)

- **Source:** [@QingQ77](https://x.com/i/status/2117161286060425550) (Geek Lite), November 1, 2026, 11:46 PM PST
- **Also:** [@LLMpsycho](https://x.com/i/status/2121246997298794817) (Bessi S), November 13, 2026, 6:21 AM PST
- **Summary:** Both posts point at [wici-ai/inferssd](https://github.com/wici-ai/inferssd), described in the linked repo tagline as a llama.cpp-based way to run models larger than RAM or VRAM by streaming MoE experts from SSD, with GGUF support. The November 1 public summary adds that it targets standard GGUFs of 763B–816B-class models on MacBooks (Metal) and NVIDIA GPUs (CUDA). The November 13 post also notes an OpenAI-compatible API and cautions that tokens/sec still need to be checked.
- **Referenced URLs:** [github.com/wici-ai/inferssd](https://github.com/wici-ai/inferssd)
- **Applies to spiro:** Weight I/O in `uie-loader-gguf` and later MoE ops in the graph executor. Today the GGUF path assumes weights are available to the session; expert streaming / mmap-style offload is a natural follow-on once `Session` actually executes ops, including a Metal or CUDA backend plugin.

### Keep large MoE “tandem” on GPU plus host RAM (tandem.cpp)

- **Source:** [@kaimi_local](https://x.com/i/status/2117144643486056951) (kaimi), November 1, 2026, 10:39 PM PST
- **Summary:** The public summary reports **tandem.cpp**, a Linux/CUDA llama.cpp patch that ran Qwen3.8-Flash-Next IQ3_S (78 GiB) at 97 tok/s on an RTX 4090 with 64 GB DDR4, versus 30 tok/s on stock llama.cpp, while computing every expert; gpt-oss-120b is cited as going from 28 to 52 tok/s. A screenshot note attributes the 29.8 vs 97.3 tok/s bar chart to the tandem.cpp README (GSM8K, greedy), not to the post author.
- **Referenced URLs:** [github.com/ewhacc/tandem.cpp](https://github.com/ewhacc/tandem.cpp)
- **Applies to spiro:** MoE expert placement across `CpuBackend` and a future CUDA backend — specifically, splitting experts between device memory and host RAM without skipping experts. Useful when wiring `Backend::execute` for MoE ops and for GGUF IQ-quant types already common in llama.cpp weights.

### Datacenter-style MoE serving on a gaming GPU (FreeToken)

- **Source:** [@BRKoshiya](https://x.com/i/status/2120195759681446042) (B Koshiya), November 10, 2026, 8:43 AM PST
- **Also:** [@TheArthiAI](https://x.com/i/status/2117161591053693422) (The Arthi AI Collective), November 1, 2026, 11:47 PM PST
- **Summary:** Both posts link [FlashML-org/FreeToken](https://github.com/FlashML-org/FreeToken). The repo tagline promises “datacenter-scale model serving” on a desktop. The November 10 public pull quote attributes the project to UC Berkeley researchers and frames it as serving giant MoE models on consumer gaming GPUs. The November 1 summary only notes that the project was trending (about 15.6k stars).
- **Referenced URLs:** [github.com/FlashML-org/FreeToken](https://github.com/FlashML-org/FreeToken)
- **Applies to spiro:** Serving and scheduling in `uie-serve` plus MoE execution in `uie-core`, once a GPU backend exists. Treat as a reference design for packing large MoE inference onto one consumer card, not as something to vendor in.

## KV cache, paged attention, and prefix reuse

### Paged attention, from OS-style paging, with a small benchmarked implementation

- **Source:** [@praty02](https://x.com/i/status/2120136967400931607) (Pratyush Ranjan), November 10, 2026, 4:50 AM PST
- **Summary:** The author shares [pratyushranjan2/Paged-Attention](https://github.com/pratyushranjan2/Paged-Attention) (“An implementation for paged attention with benchmarks”) and a blog post titled *Paged Attention Explained: Build Mini Version and Measure It*. The public pull quote says paged attention takes inspiration from virtual paging in an OS to reduce memory wastage and increase throughput.
- **Referenced URLs:** [github.com/pratyushranjan2/Paged-Attention](https://github.com/pratyushranjan2/Paged-Attention), [Paged Attention Explained: Build Mini Version and Measure It](https://ai-inference-pratyush.hashnode.dev/paged-attention-in-ai-inference)
- **Applies to spiro:** KV layout inside `Session` / the graph executor. Before attention ops run for real, decide whether the cache is a monolithic tensor or paged blocks so later batching in `uie-serve` does not force huge pre-allocations.

### A from-scratch C++/CUDA mini-engine that ends at paged attention

- **Source:** [@zyvre_ic](https://x.com/i/status/2119878964529315999) (Zyvren), November 9, 2026, 11:45 AM PST
- **Summary:** The post points at [samir-sarwar/samir-vllm](https://github.com/samir-sarwar/samir-vllm), a small LLM inference engine built from scratch with hours of video and handwritten notes. The public summary says it runs Llama 3.2 1B and is filed against local inference; the repo title in the URL metadata is “building a small scale inference engine by hand.” The pull quotes frame it as answering “what happens between your prompt and the next token” by building a tiny engine in C++ and CUDA.
- **Referenced URLs:** [github.com/samir-sarwar/samir-vllm](https://github.com/samir-sarwar/samir-vllm)
- **Applies to spiro:** A compact walkthrough of prefill, decode, and paged attention that can inform how `ExecutionPlan` ops and a CUDA (or CPU) backend should be sequenced once the graph executor is wired.

### MTP speculative decoding, FP8 KV cache, and prefix caching in a multi-GPU vLLM serve

- **Source:** [@vllm_project](https://x.com/i/status/2122472292693451062) (vLLM), November 16, 2026, 3:30 PM PST
- **Also:** [@mswnlz](https://x.com/i/status/2121636496704778653) (M.en), November 14, 2026, 8:08 AM PST
- **Summary:** The vLLM account links NVIDIA’s guide *[When one GPU isn’t enough: serving GLM-5.3 across two NVIDIA DGX Stations with vLLM](https://developer.nvidia.com/blog/when-one-gpu-isnt-enough-serving-glm-5-3-across-two-nvidia-dgx-stations-with-vllm/)* and a YouTube walkthrough, for GLM-5.3 NVFP4. A public pull quote on the earlier post states that speed comes from MTP speculative decoding, an FP8 KV cache, and prefix caching, with pinned image and checkpoint versions for reproduction. Keywords on both records include NVFP4, speculative decoding, and KV cache.
- **Referenced URLs:** [NVIDIA developer blog](https://developer.nvidia.com/blog/when-one-gpu-isnt-enough-serving-glm-5-3-across-two-nvidia-dgx-stations-with-vllm/), [YouTube walkthrough](https://www.youtube.com/watch?v=opRXPRnSkxk)
- **Applies to spiro:** Three session-level features to design for even if multi-GPU TP is far off: (1) a quantized KV cache dtype in `uie-core`, (2) prefix-cache reuse across `uie-serve` requests, (3) multi-token / MTP speculative decode beside `uie-llm` greedy completion. NVFP4 also hints that weight dtypes in the GGUF (and later SafeTensors/ONNX) loaders will need to grow past the types used today.

### Reusable KV as memory, not just a prefix cache (KVUse)

- **Source:** [@cv_usk](https://x.com/i/status/2124470699985449036) (cv usk), November 22, 2026, 3:51 AM PST
- **Summary:** The public record points to the arXiv paper *KVUse: Reusable-KV User Memory for Efficient LLM Recommenders* ([2611.21920](https://arxiv.org/pdf/2611.21920)). Figures cited in the public summary include 4x, 6.8M, and Llama-3.1-8B. Keywords include KV cache, LoRA, and distillation. (The Trouble public facet does not include a longer technical abstract, so no further claims are added here.)
- **Referenced URLs:** [arxiv.org/pdf/2611.21920](https://arxiv.org/pdf/2611.21920)
- **Applies to spiro:** A reminder that prefix caching in `uie-serve` is not the only KV-reuse problem: some workloads want to stash and reload KV segments even when the surrounding prompt changes. Worth reading before freezing the `Session` cache API.

## Quantization and speculative decoding

### 2-bit quantization-aware distillation plus a speculative drafter (Nemotron 3.5 Bolt)

- **Source:** [@anemll](https://x.com/i/status/2115867579147559383) (Anemll), October 29, 2026, 11:05 AM PDT
- **Summary:** The public summary discusses NVIDIA’s paper on **Nemotron 3.5 Bolt 74B-A9B** ([arXiv 2610.33039](https://arxiv.org/abs/2610.33039)), a hybrid Mamba-Transformer cut down from Nemotron 3.5 Super VL 120B-A12B with 26B tokens of 2-bit quantization-aware distillation so it fits in 24GB, shipping a speculative drafter (84 tok/s). The author also notes failed personal attempts at good 2-bit quality without large compute. Keywords include 2-bit quantization, quantization-aware distillation, and Mamba.
- **Referenced URLs:** [arxiv.org/abs/2610.33039](https://arxiv.org/abs/2610.33039)
- **Applies to spiro:** Two tracks: support for very-low-bit weights (and hybrid Mamba-Transformer ops) in loaders / `OpKind`, and a first-class speculative drafter next to `uie-llm` completions. The 24GB / DGX Spark framing matches the local-models topic Trouble was curated around.

### Quantizing a draft model’s KV cache (and why that may not speed serving)

- **Source:** [@RahulChaube_](https://x.com/i/status/2122729590322090366) (Rahul Chaube), November 17, 2026, 8:32 AM PST
- **Summary:** The post points to *Memory-Efficient Speculative Decoding with Quantized Draft KV Cache* ([arXiv 2611.20565](https://arxiv.org/abs/2611.20565)). Public pull quotes say quantizing the draft model’s KV cache cut its storage by 66% across 6 model pairs, with little loss in token acceptance, but the prototype was slower and peak GPU memory stayed unchanged.
- **Referenced URLs:** [arxiv.org/abs/2611.20565](https://arxiv.org/abs/2611.20565)
- **Applies to spiro:** A design warning for any speculative-decoding path in `uie-llm`: shrinking the drafter KV is not automatically a win on latency or peak memory. Acceptance-test telemetry (TTFT, tok/s, peak bytes) should measure both, not only cache size.

### Leech-lattice vector quantization for on-device LLMs

- **Source:** [@QCOMResearch](https://x.com/i/status/2118837018989834247) (Qualcomm Research & Technologies), November 6, 2026, 2:44 PM PST
- **Summary:** Qualcomm Research announces a NeurIPS 2026 paper, *Leech Lattice Vector Quantization for Efficient LLM Compression* ([arXiv 2603.11021](https://arxiv.org/abs/2603.11021)), described in the public pull quotes as compressing LLMs with the Leech lattice for on-device inference.
- **Referenced URLs:** [arxiv.org/abs/2603.11021](https://arxiv.org/abs/2603.11021)
- **Applies to spiro:** A candidate quant scheme for `uie-loader-gguf` or a later custom-quant path, especially if UIE grows an on-device / CPU or NPU backend. No need to implement it now; record it as a format the loader and `DataType` story may have to host.

### Blockwise flow LMs with self-speculative decoding (Lafa)

- **Source:** [@cv_usk](https://x.com/i/status/2115743193350168923) (cv usk), October 29, 2026, 2:51 AM PDT
- **Summary:** The public summary explains *Large Flow Language Models* ([arXiv 2610.28559](https://arxiv.org/pdf/2610.28559)): converting pretrained autoregressive models into **Lafa**, a blockwise flow LM trained on under 30B tokens that reaches 2.54× AR throughput at 7B with comparable quality, using asynchronous per-token flow times and self-speculative decoding with the retained AR head as verifier.
- **Referenced URLs:** [arxiv.org/pdf/2610.28559](https://arxiv.org/pdf/2610.28559)
- **Applies to spiro:** Self-speculative decoding (the model’s own AR head as verifier) is a decoder-side optimization the graph executor / `uie-llm` could grow without a second draft model. The flow-LM training recipe itself is out of scope unless UIE starts hosting training.

### EXL3 low-bit quants with “exact” speculative decoding (TensorFold)

- **Source:** [@bei351009016](https://x.com/i/status/2116719214350459152) (傲慢), October 31, 2026, 7:29 PM PDT
- **Summary:** The public summary notes a run of DeepSeek V4.1 Flash on two DGX Sparks with an EXL3 2.8-bit quant at 100+ tok/s single-stream and about 95% code agreement with the original, with four concurrent streams totaling only 137 tok/s. The linked repo tagline for [jayleaton/deepseek-v41-tensorfold-spark](https://github.com/jayleaton/deepseek-v41-tensorfold-spark) claims 1.8–1.9× code, 2.5× multi-stream, and 1.7–1.95× prefill versus a vLLM kit, plus “exact speculative decoding,” and marks the work in progress.
- **Referenced URLs:** [github.com/jayleaton/deepseek-v41-tensorfold-spark](https://github.com/jayleaton/deepseek-v41-tensorfold-spark)
- **Applies to spiro:** Another data point that low-bit formats beyond GGUF (EXL3) and “exact” speculative decoding show up in Spark-class local serving. Multi-stream scaling (100+ tok/s down to a shared 137 tok/s at four streams) is relevant to `uie-serve` batching goals.

## Kernels, backends, and serving methodology

### WebGPU GEMM with in-kernel 4/8-bit dequant and flash attention

- **Source:** [@JoshHughes___](https://x.com/i/status/2116335245318644016) (jhug), October 30, 2026, 6:03 PM PDT
- **Summary:** The public summary introduces Local Model Access (LMA): a phone stores one model file, sites verify it against a Hugging Face SHA-256 and stream it to the GPU, Apache-2.0 code and a CC BY 4.0 spec, tested on iPhone with Nemotron 3.5 Bolt E4B and Gemma 4 E2B. Linked repos are [ArtistJoshHughes/local-model-access](https://github.com/ArtistJoshHughes/local-model-access) and [ArtistJoshHughes/p0pc0rn-kernels](https://github.com/ArtistJoshHughes/p0pc0rn-kernels). The kernels repo tagline in the URL metadata is: WebGPU compute kernels for image, video and language models in the browser — GEMM with in-kernel 4/8-bit dequantisation, flash attention, 3D convolution, tuned on iPhone.
- **Referenced URLs:** [github.com/ArtistJoshHughes/local-model-access](https://github.com/ArtistJoshHughes/local-model-access), [github.com/ArtistJoshHughes/p0pc0rn-kernels](https://github.com/ArtistJoshHughes/p0pc0rn-kernels)
- **Applies to spiro:** Directly relevant to the README’s future CUDA / WASM backends (and any WebGPU plugin). In-kernel dequant + flash attention is the kind of `Backend` work `CpuBackend` will eventually need a sibling for; LMA’s “one on-device file, many clients” pattern is closer to `uie-serve` than to the core executor.

### Custom kernels, quantization, and speculative decoding as a vLLM plugin (atdev-serve)

- **Source:** [@atdevcompany](https://x.com/i/status/2117961582995206635) (ATDEV), November 4, 2026, 4:46 AM PST
- **Summary:** ATDEV open-sources atdev-serve, a vLLM plugin that speeds up Qwen3.8-27B with custom kernels, quantization, speculative decoding, and serving tweaks. Public pull quotes claim up to 14.2× faster inference on a single A100 80GB. The linked repo is [atdev-company/atdev-serve](https://github.com/atdev-company/atdev-serve) (described as a drop-in plugin for vLLM 0.31.0).
- **Referenced URLs:** [github.com/atdev-company/atdev-serve](https://github.com/atdev-company/atdev-serve)
- **Applies to spiro:** Not a vLLM port. The interesting part is the combination — custom kernels + quant + speculative decode + serving — as a checklist for when `uie-llm` / a GPU backend starts to exist. Treat claimed speedups as vendor numbers to verify, not as targets.

### Hill-climb local serving settings (decode, speculative, prefill, KV, startup, TTFT)

- **Source:** [@AIDailyGems](https://x.com/i/status/2122625874680058122) (AIDailyGems), November 17, 2026, 1:40 AM PST
- **Also (same repo, not re-summarized):** [@Z3nlyte](https://x.com/i/status/2122725376279421125), November 17, 2026, 8:15 AM PST; [@davidpereIsHIM](https://x.com/i/status/2123144676345532438), November 18, 2026, 12:01 PM PST; [@CatMaster20000](https://x.com/i/status/2123163906914836641), November 18, 2026, 1:18 PM PST; [@kumaaa060b](https://x.com/i/status/2123433678898413951), November 19, 2026, 7:10 AM PST
- **Summary:** The primary post shares [sybil-solutions/inference-optimisation-skill](https://github.com/sybil-solutions/inference-optimisation-skill), an agent skill for hill-climbing local LLM serving speed (decode, speculative decoding, prefill, KV, startup, TTFT) on any hardware and quantization. It mentions RTX 3090 and Qwen3.8-27B, and cites 96.2 tok/s on prose / 141.1 tok/s on code (MIT). Later posts repeat the same repo; one public quote claims the authors more than doubled prefill speed at 256K context on the same hardware; another points at B390 numbers and a hill-climb log in local-ai-registry PR #453.
- **Referenced URLs:** [github.com/sybil-solutions/inference-optimisation-skill](https://github.com/sybil-solutions/inference-optimisation-skill)
- **Applies to spiro:** Methodology more than a kernel. Once `uie-llm` / `uie-serve` can complete a prompt, a measured, one-variable-at-a-time loop over prefill, decode, KV, and startup (with the existing acceptance-test telemetry) is how to avoid folklore tuning. Not a product feature to import as a skill.

### A hands-on inference-optimization handbook

- **Source:** [@Shoumya23](https://x.com/i/status/2123405923159015443) (Shoumya Vishwakarma), November 19, 2026, 5:19 AM PST
- **Summary:** The post shares [TensorTonic/llm-optimization-handbook](https://github.com/TensorTonic/llm-optimization-handbook). The repo tagline in the URL metadata is: make LLMs faster, cheaper and smaller — inference fundamentals, quantization, KV cache, speculative decoding, GPU kernels and serving, with hands-on problems. The public text says it covers 48 topics including quantization, KV cache, GPU kernels, batching, and serving.
- **Referenced URLs:** [github.com/TensorTonic/llm-optimization-handbook](https://github.com/TensorTonic/llm-optimization-handbook)
- **Applies to spiro:** A structured syllabus that lines up with UIE’s remaining work (executor, KV, kernels, serving). Useful as a reading list when picking the next `Op` or backend story; not itself a patch.

### Prefill vs decode, and the usual serving levers (cheatsheet)

- **Source:** [@ai_radar_](https://x.com/i/status/2120392776336724074) (AI Radar 📡), November 10, 2026, 9:46 PM PST
- **Summary:** The post links an “LLM Inference Explained” cheatsheet on [airadar08.github.io/cheatsheets](https://airadar08.github.io/cheatsheets/#llm-inference-explained). Public pull quotes list prefill vs decode, TTFT / TPOT / throughput, KV cache, batching, speculative decoding, and quantization.
- **Referenced URLs:** [AI Radar Cheatsheets — LLM Inference Explained](https://airadar08.github.io/cheatsheets/#llm-inference-explained)
- **Applies to spiro:** A compact vocabulary for `uie-llm` / `uie-serve` telemetry and for how the graph executor should split prefill and decode once `Session` I/O is wired to `Backend::execute`.

### Compare engines on init time as well as tok/s (vLLM+drafter, SGLang, llama.cpp)

- **Source:** [@GarryLing1](https://x.com/i/status/2123582225530908678) (Anaxagore), November 19, 2026, 5:00 PM PST
- **Summary:** A public write-up compared five serving engines on one NVIDIA L4. The post mentions vLLM, llama.cpp, and SGLang, and cites 275.3 tok/s and 554.8 tok/s. A pull quote says the bench (by xbill9) used Gemma 4 E2B in Q4 QAT / GGUF / BF16, 256 greedy tokens, on 1x L4. Linked material includes an AWS Builder Center article, *Five Serving Engines on a Lambda GPU: Gemma 4 E2B Under vLLM, SGLang, llama.cpp, PyTorch and JAX*, and [xbill9/gemma4-dev](https://github.com/xbill9/gemma4-dev). Keywords include llama.cpp, vLLM, and SGLang. The public facet does not restate every engine’s init-time figure, so those numbers are not copied from the raw post text.
- **Referenced URLs:** [AWS Builder Center article](https://builder.aws.com/content/3MLBaBQTArQRiKgsJQhl9L77OSy/five-serving-engines-on-a-lambda-gpu-gemma-4-e2b-under-vllm-sglang-llamacpp-pytorch-and-jax), [github.com/xbill9/gemma4-dev](https://github.com/xbill9/gemma4-dev)
- **Applies to spiro:** When `uie-llm` is benchmarked against llama.cpp or vLLM, measure cold-start / init and concurrency, not only decode tok/s. GGUF vs BF16 vs Q4-QAT in one harness is a realistic acceptance-test matrix for loaders.

## Reviewed and not kept as primary items

These were read in full but are not listed above as new techniques: a GGUF visual workbench ([GGUF Atlas](https://github.com/llmxlabs/gguf-atlas), [@llmxlabs](https://x.com/i/status/2119265167041282176)); a Q6_K GGUF model release ([@FahlSolotov](https://x.com/i/status/2119100672301760948)); a llama.cpp + Flash Attention bench of one GGUF ([@blue0orbit](https://x.com/i/status/2122470286817677590)); an agent-memory skill that is harness context, not inference ([@arthur6892](https://x.com/i/status/2120290482014376202)); an MLX 4/6/8-bit model port ([@ParisMichalas](https://x.com/i/status/2122761865923379597)) — useful later for an Apple/Metal backend, but not a new algorithm; a CPU-only LFM3 GGUF llama.cpp bench ([@sakikoazu](https://x.com/i/status/2116867578572447943)); and a short note pointing at llama.cpp’s quantize README ([@EricGail](https://x.com/i/status/2120878862682701986)). Duplicate posts of the inference-optimisation-skill repo are recorded under that item rather than as separate ideas.
