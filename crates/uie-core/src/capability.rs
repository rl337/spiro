//! Capability probes and safe kernel dispatch (US-029).
//!
//! Prefer probing before selecting an optimized path — never assume NEON/dotprod
//! (or assume a Pi GPU / NPU) and trap.

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use crate::error::{Error, Result};

/// CPU (and later accelerator) features that may gate a fast path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CpuFeature {
    Neon,
    DotProd,
    /// Placeholder for x86 CI hosts; not required on ARM.
    Avx2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelPath {
    Scalar,
    Optimized { feature: CpuFeature },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchDecision {
    pub path: KernelPath,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityEvent {
    Probe { feature: CpuFeature, present: bool },
    Dispatch { path: KernelPath, reason: String },
    Fallback { missing: CpuFeature, reason: String },
    Unsupported { feature: CpuFeature },
}

/// Detects features before optimized dispatch (injectable; fake in CI).
pub trait CapabilityProbe: Send + Sync {
    fn has(&self, feature: CpuFeature) -> bool;
    fn take_events(&self) -> Vec<CapabilityEvent> {
        Vec::new()
    }
}

/// Always reports the configured feature set (US-029 CI).
pub struct FakeCapabilityProbe {
    features: BTreeSet<CpuFeature>,
    events: Mutex<Vec<CapabilityEvent>>,
}

impl FakeCapabilityProbe {
    pub fn with_features(features: impl IntoIterator<Item = CpuFeature>) -> Arc<Self> {
        Arc::new(Self {
            features: features.into_iter().collect(),
            events: Mutex::new(Vec::new()),
        })
    }

    pub fn empty() -> Arc<Self> {
        Self::with_features([])
    }

    fn push(&self, ev: CapabilityEvent) {
        if let Ok(mut g) = self.events.lock() {
            g.push(ev);
        }
    }
}

impl CapabilityProbe for FakeCapabilityProbe {
    fn has(&self, feature: CpuFeature) -> bool {
        let present = self.features.contains(&feature);
        self.push(CapabilityEvent::Probe { feature, present });
        present
    }

    fn take_events(&self) -> Vec<CapabilityEvent> {
        self.events
            .lock()
            .map(|mut g| std::mem::take(&mut *g))
            .unwrap_or_default()
    }
}

/// Select a kernel path. Missing preferred feature → scalar fallback (never illegal insn).
pub fn select_kernel(
    probe: &dyn CapabilityProbe,
    preferred: Option<CpuFeature>,
) -> DispatchDecision {
    match preferred {
        None => {
            let d = DispatchDecision {
                path: KernelPath::Scalar,
                reason: "no optimized path requested".into(),
            };
            d
        }
        Some(feature) if probe.has(feature) => DispatchDecision {
            path: KernelPath::Optimized { feature },
            reason: format!("feature {feature:?} present"),
        },
        Some(feature) => DispatchDecision {
            path: KernelPath::Scalar,
            reason: format!("fallback: feature {feature:?} unavailable"),
        },
    }
}

/// Like [`select_kernel`], but records fallback/unsupported on a fake probe's event log when possible.
pub fn select_kernel_traced(
    probe: &FakeCapabilityProbe,
    preferred: Option<CpuFeature>,
) -> DispatchDecision {
    let decision = select_kernel(probe, preferred);
    match &decision.path {
        KernelPath::Optimized { .. } => {
            probe.push(CapabilityEvent::Dispatch {
                path: decision.path.clone(),
                reason: decision.reason.clone(),
            });
        }
        KernelPath::Scalar => {
            if let Some(feature) = preferred {
                if !probe.features.contains(&feature) {
                    probe.push(CapabilityEvent::Fallback {
                        missing: feature,
                        reason: decision.reason.clone(),
                    });
                } else {
                    probe.push(CapabilityEvent::Dispatch {
                        path: decision.path.clone(),
                        reason: decision.reason.clone(),
                    });
                }
            } else {
                probe.push(CapabilityEvent::Dispatch {
                    path: decision.path.clone(),
                    reason: decision.reason.clone(),
                });
            }
        }
    }
    decision
}

/// Explicit unsupported when a caller required a feature with no fallback allowed.
pub fn require_feature(probe: &dyn CapabilityProbe, feature: CpuFeature) -> Result<()> {
    if probe.has(feature) {
        Ok(())
    } else {
        Err(Error::UnsupportedOp(format!(
            "required CPU feature {feature:?} not available (US-029)"
        )))
    }
}

/// Deterministic fill used to compare scalar vs "optimized" (same math in CI).
pub fn fill_f32_scalar(dst: &mut [f32], value: f32) {
    for x in dst {
        *x = value;
    }
}

/// "Optimized" fill — identical result; gated by probe so CI can assert dispatch.
pub fn fill_f32_dispatch(
    probe: &dyn CapabilityProbe,
    preferred: Option<CpuFeature>,
    dst: &mut [f32],
    value: f32,
) -> DispatchDecision {
    let decision = select_kernel(probe, preferred);
    // Both paths use the same scalar fill today — correctness first; real NEON later.
    fill_f32_scalar(dst, value);
    decision
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_feature_falls_back_to_scalar() {
        let probe = FakeCapabilityProbe::empty();
        let d = select_kernel_traced(&probe, Some(CpuFeature::Neon));
        assert_eq!(d.path, KernelPath::Scalar);
        let ev = probe.take_events();
        assert!(ev.iter().any(|e| matches!(e, CapabilityEvent::Fallback { .. })));
    }

    #[test]
    fn present_feature_selects_optimized() {
        let probe = FakeCapabilityProbe::with_features([CpuFeature::Neon]);
        let d = select_kernel_traced(&probe, Some(CpuFeature::Neon));
        assert_eq!(
            d.path,
            KernelPath::Optimized {
                feature: CpuFeature::Neon
            }
        );
    }

    #[test]
    fn require_feature_errors_explicitly() {
        let probe = FakeCapabilityProbe::empty();
        let err = require_feature(probe.as_ref(), CpuFeature::DotProd).unwrap_err();
        assert!(matches!(err, Error::UnsupportedOp(_)));
    }

    #[test]
    fn scalar_and_optimized_fill_match() {
        let probe = FakeCapabilityProbe::with_features([CpuFeature::Neon]);
        let mut a = [0f32; 7]; // odd length / tail
        let mut b = [0f32; 7];
        fill_f32_scalar(&mut a, 1.25);
        let _ = fill_f32_dispatch(probe.as_ref(), Some(CpuFeature::Neon), &mut b, 1.25);
        assert_eq!(a, b);
    }
}
