//! GGUF GPU offload helpers (US-030 experiment).
//!
//! Pi targets **Vulkan** (`AcceleratorKind::PiGpu`). Apple Silicon / CUDA hosts report
//! [`AcceleratorKind::HostGpu`] so we can exercise `n_gpu_layers` before Pi hardware.

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::{list_llama_ggml_backend_devices, LlamaBackendDeviceType};
use uie_core::accelerator::{AcceleratorEvent, AcceleratorKind, AcceleratorProbe};

/// How many transformer layers to place on the GPU (`0` = CPU-only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GgufGpuOffload {
    pub n_gpu_layers: u32,
    pub kind: Option<AcceleratorKind>,
}

impl GgufGpuOffload {
    pub fn cpu_only() -> Self {
        Self {
            n_gpu_layers: 0,
            kind: None,
        }
    }

    pub fn from_probe(probe: &dyn AcceleratorProbe, layers_when_present: u32) -> Self {
        match AcceleratorKind::preferred_gguf_offload(probe) {
            Some(kind) => Self {
                n_gpu_layers: layers_when_present,
                kind: Some(kind),
            },
            None => Self::cpu_only(),
        }
    }
}

/// Classify ggml devices into UIE accelerator kinds (no silent GPU claim).
pub fn classify_ggml_devices(
    devices: impl IntoIterator<Item = llama_cpp_2::LlamaBackendDevice>,
) -> BTreeSet<AcceleratorKind> {
    let mut kinds = BTreeSet::new();
    for d in devices {
        let is_gpu = matches!(
            d.device_type,
            LlamaBackendDeviceType::Gpu | LlamaBackendDeviceType::IntegratedGpu
        );
        if !is_gpu {
            continue;
        }
        let backend = d.backend.to_ascii_lowercase();
        if backend.contains("vulkan") {
            kinds.insert(AcceleratorKind::PiGpu);
        } else if backend.contains("mtl")
            || backend.contains("metal")
            || backend.contains("cuda")
            || backend.contains("hip")
            || backend.contains("rocm")
        {
            kinds.insert(AcceleratorKind::HostGpu);
        }
    }
    kinds
}

/// Live probe: init llama backend, list ggml devices, map Vulkan→PiGpu / Metal|CUDA→HostGpu.
pub struct LiveGgmlAcceleratorProbe {
    present: BTreeSet<AcceleratorKind>,
    events: Mutex<Vec<AcceleratorEvent>>,
    supports_gpu_offload: bool,
}

impl LiveGgmlAcceleratorProbe {
    pub fn detect() -> Arc<Self> {
        let supports_gpu_offload = LlamaBackend::init()
            .map(|b| b.supports_gpu_offload())
            .unwrap_or(false);
        let devices = list_llama_ggml_backend_devices();
        let present = classify_ggml_devices(devices);
        Arc::new(Self {
            present,
            events: Mutex::new(Vec::new()),
            supports_gpu_offload,
        })
    }

    /// Human-readable ggml device lines for `uie-llm list-gpu`.
    pub fn format_device_lines() -> Vec<String> {
        let _ = LlamaBackend::init();
        list_llama_ggml_backend_devices()
            .into_iter()
            .map(|d| {
                format!(
                    "{}\t{}\t{:?}\t{}",
                    d.backend, d.name, d.device_type, d.description
                )
            })
            .collect()
    }

    pub fn supports_gpu_offload(&self) -> bool {
        self.supports_gpu_offload
    }

    pub fn present_kinds(&self) -> &BTreeSet<AcceleratorKind> {
        &self.present
    }

    fn push(&self, ev: AcceleratorEvent) {
        if let Ok(mut g) = self.events.lock() {
            g.push(ev);
        }
    }
}

impl AcceleratorProbe for LiveGgmlAcceleratorProbe {
    fn has(&self, kind: AcceleratorKind) -> bool {
        let present = self.present.contains(&kind);
        self.push(AcceleratorEvent::Probe { kind, present });
        present
    }

    fn take_events(&self) -> Vec<AcceleratorEvent> {
        self.events
            .lock()
            .map(|mut g| std::mem::take(&mut *g))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use llama_cpp_2::LlamaBackendDevice;

    fn fake_dev(backend: &str, device_type: LlamaBackendDeviceType) -> LlamaBackendDevice {
        LlamaBackendDevice {
            index: 0,
            name: "t".into(),
            description: String::new(),
            backend: backend.into(),
            memory_total: 0,
            memory_free: 0,
            device_type,
        }
    }

    #[test]
    fn vulkan_maps_to_pi_gpu() {
        let kinds = classify_ggml_devices([fake_dev("Vulkan", LlamaBackendDeviceType::Gpu)]);
        assert!(kinds.contains(&AcceleratorKind::PiGpu));
        assert!(!kinds.contains(&AcceleratorKind::HostGpu));
    }

    #[test]
    fn metal_maps_to_host_gpu() {
        let kinds = classify_ggml_devices([fake_dev("MTL", LlamaBackendDeviceType::Gpu)]);
        assert!(kinds.contains(&AcceleratorKind::HostGpu));
        assert!(!kinds.contains(&AcceleratorKind::PiGpu));
    }

    #[test]
    fn cpu_device_ignored() {
        let kinds = classify_ggml_devices([fake_dev("CPU", LlamaBackendDeviceType::Cpu)]);
        assert!(kinds.is_empty());
    }
}
