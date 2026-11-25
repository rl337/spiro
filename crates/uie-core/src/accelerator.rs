//! Accelerator presence probes (US-030 Pi GPU, US-031 NPU).
//!
//! CI uses fakes; live silicon is opt-in. Missing hardware → explicit fallback / unsupported.

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use crate::backend::{Backend, BackendId, Device, DeviceKind};
use crate::error::{Error, Result};
use crate::ops::Op;
use crate::tensor::{DataType, Shape, Tensor};

/// On-device accelerators beyond host CPU (US-030 / US-031).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AcceleratorKind {
    /// Raspberry Pi GPU (Vulkan / vendor path later).
    PiGpu,
    /// Neural processing unit / similar dedicated inference silicon.
    Npu,
}

impl AcceleratorKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PiGpu => "pi_gpu",
            Self::Npu => "npu",
        }
    }

    pub fn backend_id(self) -> BackendId {
        BackendId::new(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AcceleratorEvent {
    Probe {
        kind: AcceleratorKind,
        present: bool,
    },
    Select {
        kind: AcceleratorKind,
    },
    Fallback {
        kind: AcceleratorKind,
        reason: String,
    },
    Unsupported {
        kind: AcceleratorKind,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AcceleratorChoice {
    /// Use host CPU (default / fallback).
    Cpu,
    Accelerator(AcceleratorKind),
}

/// Detects accelerator availability (injectable; fake in CI).
pub trait AcceleratorProbe: Send + Sync {
    fn has(&self, kind: AcceleratorKind) -> bool;
    fn take_events(&self) -> Vec<AcceleratorEvent> {
        Vec::new()
    }
}

/// Always reports the configured accelerator set (US-030 / US-031 CI).
pub struct FakeAcceleratorProbe {
    present: BTreeSet<AcceleratorKind>,
    events: Mutex<Vec<AcceleratorEvent>>,
}

impl FakeAcceleratorProbe {
    pub fn with_kinds(kinds: impl IntoIterator<Item = AcceleratorKind>) -> Arc<Self> {
        Arc::new(Self {
            present: kinds.into_iter().collect(),
            events: Mutex::new(Vec::new()),
        })
    }

    pub fn empty() -> Arc<Self> {
        Self::with_kinds([])
    }

    fn push(&self, ev: AcceleratorEvent) {
        if let Ok(mut g) = self.events.lock() {
            g.push(ev);
        }
    }
}

impl AcceleratorProbe for FakeAcceleratorProbe {
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

/// Prefer `preferred` when present; otherwise CPU fallback (never pretend accelerator ran).
pub fn select_accelerator(
    probe: &dyn AcceleratorProbe,
    preferred: Option<AcceleratorKind>,
) -> AcceleratorChoice {
    match preferred {
        Some(kind) if probe.has(kind) => AcceleratorChoice::Accelerator(kind),
        Some(_) | None => AcceleratorChoice::Cpu,
    }
}

pub fn select_accelerator_traced(
    probe: &FakeAcceleratorProbe,
    preferred: Option<AcceleratorKind>,
) -> AcceleratorChoice {
    let choice = select_accelerator(probe, preferred);
    match (&choice, preferred) {
        (AcceleratorChoice::Accelerator(kind), _) => {
            probe.push(AcceleratorEvent::Select { kind: *kind });
        }
        (AcceleratorChoice::Cpu, Some(kind)) => {
            probe.push(AcceleratorEvent::Fallback {
                kind,
                reason: format!("{kind:?} unavailable; using CPU"),
            });
        }
        (AcceleratorChoice::Cpu, None) => {}
    }
    choice
}

/// Hard require — no CPU fallback (explicit unsupported).
pub fn require_accelerator(probe: &dyn AcceleratorProbe, kind: AcceleratorKind) -> Result<()> {
    if probe.has(kind) {
        Ok(())
    } else {
        Err(Error::UnsupportedOp(format!(
            "required accelerator {kind:?} not available (US-030/US-031)"
        )))
    }
}

/// CI stub backend: same fill ops as CPU, but reports [`DeviceKind::Accelerator`].
///
/// Real Pi GPU / NPU kernels stay hardware-gated; this proves DI + probe dispatch.
#[derive(Debug)]
pub struct StubAcceleratorBackend {
    kind: AcceleratorKind,
    id: BackendId,
}

impl StubAcceleratorBackend {
    pub fn new(kind: AcceleratorKind) -> Self {
        Self {
            id: kind.backend_id(),
            kind,
        }
    }

    pub fn kind(&self) -> AcceleratorKind {
        self.kind
    }
}

impl Backend for StubAcceleratorBackend {
    fn id(&self) -> BackendId {
        self.id.clone()
    }

    fn device(&self) -> Device {
        Device {
            kind: DeviceKind::Accelerator,
            index: 0,
            label: Some(self.kind.as_str().into()),
        }
    }

    fn allocate(&self, dtype: DataType, shape: Shape) -> Result<Tensor> {
        Tensor::empty(dtype, shape)
    }

    fn execute(&self, op: &Op, inputs: &[Tensor], outputs: &mut [Tensor]) -> Result<()> {
        // Delegate semantics to CPU until real EP/Vulkan/NPU runtimes ship.
        crate::backend::cpu::CpuBackend::new().execute(op, inputs, outputs)
    }
}

/// Build a backend for a probe decision (stub accelerator or CPU).
pub fn backend_for_choice(choice: AcceleratorChoice) -> Box<dyn Backend> {
    match choice {
        AcceleratorChoice::Cpu => Box::new(crate::backend::cpu::CpuBackend::new()),
        AcceleratorChoice::Accelerator(kind) => Box::new(StubAcceleratorBackend::new(kind)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_pi_gpu_falls_back_to_cpu() {
        let probe = FakeAcceleratorProbe::empty();
        let c = select_accelerator_traced(&probe, Some(AcceleratorKind::PiGpu));
        assert_eq!(c, AcceleratorChoice::Cpu);
        let ev = probe.take_events();
        assert!(ev
            .iter()
            .any(|e| matches!(e, AcceleratorEvent::Fallback { .. })));
    }

    #[test]
    fn present_npu_selects_stub() {
        let probe = FakeAcceleratorProbe::with_kinds([AcceleratorKind::Npu]);
        let c = select_accelerator_traced(&probe, Some(AcceleratorKind::Npu));
        assert_eq!(c, AcceleratorChoice::Accelerator(AcceleratorKind::Npu));
        let b = backend_for_choice(c);
        assert_eq!(b.device().kind, DeviceKind::Accelerator);
        assert_eq!(b.device().label.as_deref(), Some("npu"));
    }

    #[test]
    fn require_accelerator_errors_explicitly() {
        let probe = FakeAcceleratorProbe::empty();
        let err = require_accelerator(probe.as_ref(), AcceleratorKind::PiGpu).unwrap_err();
        assert!(matches!(err, Error::UnsupportedOp(_)));
    }
}
