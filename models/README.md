# Bundled models

## `mnist-8.onnx`

- **Source:** [ONNX Model Zoo — MNIST](https://github.com/onnx/models/tree/main/validated/vision/classification/mnist) (`mnist-8.onnx`, ~26 KiB)
- **Task:** 28×28 grayscale digit classification (classes 0–9)
- **Input:** `float32` tensor shape `[1, 1, 28, 28]` (values typically 0.0–1.0)
- **Output:** `float32` logits shape `[1, 10]`
- **License:** Apache-2.0 (ONNX Model Zoo terms)

## `fixtures/digit_one_f32.bin`

- Raw little-endian `f32` raster (784 values) for a synthetic vertical stroke “1”, used by `uie-mnist` tests/demo.