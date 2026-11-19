# Glossary

Alphabetical list of terms used in [user stories](./stories/README.md). Anchors match link targets (`#tensor`, etc.).

---

### Backend {#backend}

A **backend** is the piece that actually runs compute: it allocates memory and (eventually) executes individual operations. UIE talks to backends through a shared interface so the same [runtime](#inference-runtime) can target CPU or accelerators later.

**See also:** [US-003](./stories/003-backends-cpu.md)

---

### CpuBackend {#cpubackend}

The **CPU backend** is the reference [backend](#backend) that runs on the host machine. It is always available and implements a small set of [operations](#operation) today (for example filling a [tensor](#tensor) with a constant).

**See also:** [US-003](./stories/003-backends-cpu.md)

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

### Inference executor {#inference-executor}

An **inference executor** is a loaded [model](#inference-model) you can run end-to-end: hand it input [tensors](#tensor) and receive outputs (for example an ONNX network). This is the path used by [model loaders](#model-loader).

**See also:** [US-007](./stories/007-models-loaders.md), [US-009](./stories/009-onnx-mnist.md)

---

### Inference model {#inference-model}

An **inference model** (plan-style) exposes metadata plus an [execution plan](#execution-plan) instead of a single `infer` call. Scaffold code still uses this for graph-first workflows.

**See also:** [US-007](./stories/007-models-loaders.md)

---

### Inference runtime {#inference-runtime}

The **inference runtime** is the front door: it owns the active [backend](#backend), registers [model loaders](#model-loader), creates [sessions](#session), and runs plans or direct inference.

**See also:** [US-004](./stories/004-runtime-sessions.md)

---

### Io spec {#io-spec}

An **I/O spec** names a model input or output: [tensor](#tensor) [shape](#shape), [data type](#datatype), and a label. Loaders use specs so callers know what to pass in.

**See also:** [US-007](./stories/007-models-loaders.md)

---

### Model loader {#model-loader}

A **model loader** knows how to read a file format (for example `.onnx`) and produce an [inference executor](#inference-executor). The [runtime](#inference-runtime) picks the first loader that recognizes a path.

**See also:** [US-007](./stories/007-models-loaders.md), [US-009](./stories/009-onnx-mnist.md)

---

### Model loader registry {#model-loader-registry}

The **model loader registry** holds every registered loader. When you ask the [runtime](#inference-runtime) to open a file, it walks the registry until one loader claims the path.

**See also:** [US-007](./stories/007-models-loaders.md)

---

### Operation {#operation}

An **operation** (op) is one step in a compute [graph](#graph): fill a buffer, apply a map, or a custom kernel. [Backends](#backend) implement ops; the CPU backend supports a minimal set while the graph executor matures.

**See also:** [US-006](./stories/006-operations.md)

---

### Session {#session}

A **session** is a workspace for one run: named [tensors](#tensor) bound to strings (like `"canvas"` or `"input"`). The [runtime](#inference-runtime) can read and write those bindings as ops or models execute.

**See also:** [US-004](./stories/004-runtime-sessions.md)

---

### Shape {#shape}

A **shape** is the list of dimensions for a [tensor](#tensor), such as `[2048, 2048, 3]` for an RGB image or `[1, 1, 28, 28]` for MNIST. Rank is the number of dimensions; element count is their product.

**See also:** [US-002](./stories/002-tensors.md)

---

### Tensor {#tensor}

A **tensor** is typed, shaped data: bytes in memory plus [shape](#shape) and [data type](#datatype). Everything you feed into UIE—or save from a demo—flows through tensors.

**See also:** [US-002](./stories/002-tensors.md)

---

### UIE {#uie}

**UIE** (Universal Inference Engine) is the Rust workspace in this repo: shared core (`uie-core`), demos (`uie-spiro`, `uie-mnist`), and loader crates (`uie-loader-onnx`).

**See also:** [US-001](./stories/001-why-uie.md)

---

### Workload {#workload}

A **workload** is any program UIE runs: a classic demo like the Spirograph, or a neural [model](#inference-model). Workloads help prove the engine pieces fit together.

**See also:** [US-008](./stories/008-spiro-demo.md)