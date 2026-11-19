# Stub manifests (`.stub`)

One line per file: modality id (see `uie_stub_models::ModalityKind`).

| File | Modality | Real-model I/O contract |
|------|----------|-------------------------|
| `llm.stub` | LLM | `input_ids` I32 `[1, seq]` → `output_ids` I32 `[1, 128]` |
| `judge.stub` | Judge | `input_ids` → `logits` F32 `[1,2]` + `label_ids` |
| `diffusion_t2i.stub` | T2I | CLIP `encoder_hidden_states` `[1,77,768]` → NCHW `[1,3,512,512]` |
| `stt.stub` | ASR | PCM F32 `[1, samples]` 16 kHz → `sequences` I32 `[1,256]` |
| `tts.stub` | TTS | `input_ids` → `waveform` F32 `[1,110250]` 22.05 kHz |
| `image_to_text.stub` | Caption | `pixel_values` `[1,3,224,224]` → `output_ids` |
| `vlm.stub` | VLM | `input_ids` + `[1,3,336,336]` → `output_ids` |
| `audio_to_audio.stub` | A2A | waveform in → same-shape silent waveform |
| `image_to_image.stub` | I2I | `[1,3,512,512]` → `[1,3,512,512]` |

Run: `cargo run -p uie-stubs -- models/stubs/llm.stub`