//! Memory admission / residency budgets (US-025).
//!
//! Domain types only — composition roots bind concrete providers via `uie-iv`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::error::{Error, Result};

/// How trustworthy a byte estimate is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EstimateConfidence {
    /// Exact known size (tests / synthetic).
    Exact,
    /// From filesystem length (weights ≈ file; workspace unknown).
    FileSize,
    /// Best-effort placeholder; consumers may treat conservatively.
    Unknown,
}

#[derive(Debug, Clone)]
pub struct MemoryEstimate {
    pub bytes: u64,
    pub confidence: EstimateConfidence,
    pub source: String,
}

impl MemoryEstimate {
    pub fn exact(bytes: u64, source: impl Into<String>) -> Self {
        Self {
            bytes,
            confidence: EstimateConfidence::Exact,
            source: source.into(),
        }
    }

    pub fn from_file_len(bytes: u64, path_label: impl Into<String>) -> Self {
        Self {
            bytes,
            confidence: EstimateConfidence::FileSize,
            source: path_label.into(),
        }
    }
}

/// Held while a load is in flight; must [`MemoryBudget::release`] or [`MemoryBudget::commit`].
#[derive(Debug)]
pub struct Reservation {
    pub id: u64,
    pub bytes: u64,
}

/// Decision recorded for observability / StoryEventLog mapping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissionEvent {
    Reserve { id: u64, bytes: u64, source: String },
    Reject { bytes: u64, remaining: u64, source: String },
    Release { id: u64, bytes: u64 },
    Commit { id: u64, bytes: u64, key: String },
    ReleaseResidency { key: String, bytes: u64 },
}

/// Capacity gate for model admission (cgroup live; fake in CI).
pub trait MemoryBudget: Send + Sync {
    fn budget_source(&self) -> &str;
    /// `None` = unlimited.
    fn capacity_bytes(&self) -> Option<u64>;
    fn remaining_bytes(&self) -> Option<u64>;
    fn try_reserve(&self, estimate: &MemoryEstimate) -> Result<Reservation>;
    fn release(&self, reservation: Reservation);
    fn commit(&self, reservation: Reservation, key: &str);
    fn release_residency(&self, key: &str);
    fn take_events(&self) -> Vec<AdmissionEvent> {
        Vec::new()
    }
}

/// Always admits (default for existing hosts / serve until a budget is injected).
#[derive(Debug, Default)]
pub struct UnlimitedMemoryBudget;

impl MemoryBudget for UnlimitedMemoryBudget {
    fn budget_source(&self) -> &str {
        "unlimited"
    }

    fn capacity_bytes(&self) -> Option<u64> {
        None
    }

    fn remaining_bytes(&self) -> Option<u64> {
        None
    }

    fn try_reserve(&self, estimate: &MemoryEstimate) -> Result<Reservation> {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        Ok(Reservation {
            id: NEXT.fetch_add(1, Ordering::Relaxed),
            bytes: estimate.bytes,
        })
    }

    fn release(&self, _reservation: Reservation) {}

    fn commit(&self, _reservation: Reservation, _key: &str) {}

    fn release_residency(&self, _key: &str) {}
}

/// Deterministic fixed-capacity budget for CI (US-025).
pub struct FakeMemoryBudget {
    capacity: u64,
    reserved: Mutex<u64>,
    committed: Mutex<HashMap<String, u64>>,
    events: Mutex<Vec<AdmissionEvent>>,
    next_id: AtomicU64,
}

impl FakeMemoryBudget {
    pub fn with_capacity(capacity: u64) -> Arc<Self> {
        Arc::new(Self {
            capacity,
            reserved: Mutex::new(0),
            committed: Mutex::new(HashMap::new()),
            events: Mutex::new(Vec::new()),
            next_id: AtomicU64::new(1),
        })
    }

    fn push(&self, ev: AdmissionEvent) {
        if let Ok(mut g) = self.events.lock() {
            g.push(ev);
        }
    }

    fn used_locked(&self, reserved: u64, committed: &HashMap<String, u64>) -> u64 {
        reserved + committed.values().sum::<u64>()
    }
}

impl MemoryBudget for FakeMemoryBudget {
    fn budget_source(&self) -> &str {
        "fake"
    }

    fn capacity_bytes(&self) -> Option<u64> {
        Some(self.capacity)
    }

    fn remaining_bytes(&self) -> Option<u64> {
        let reserved = *self.reserved.lock().ok()?;
        let committed = self.committed.lock().ok()?;
        Some(self.capacity.saturating_sub(self.used_locked(reserved, &committed)))
    }

    fn try_reserve(&self, estimate: &MemoryEstimate) -> Result<Reservation> {
        let mut reserved = self
            .reserved
            .lock()
            .map_err(|_| Error::Runtime("budget reserved lock poisoned".into()))?;
        let committed = self
            .committed
            .lock()
            .map_err(|_| Error::Runtime("budget committed lock poisoned".into()))?;
        let used = self.used_locked(*reserved, &committed);
        let remaining = self.capacity.saturating_sub(used);
        if estimate.bytes > remaining {
            self.push(AdmissionEvent::Reject {
                bytes: estimate.bytes,
                remaining,
                source: estimate.source.clone(),
            });
            return Err(Error::AdmissionRejected {
                estimate_bytes: estimate.bytes,
                remaining_bytes: remaining,
                budget_source: self.budget_source().into(),
                reason: format!(
                    "estimate {} exceeds remaining {} (capacity {}, source={})",
                    estimate.bytes,
                    remaining,
                    self.capacity,
                    estimate.source
                ),
            });
        }
        *reserved += estimate.bytes;
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.push(AdmissionEvent::Reserve {
            id,
            bytes: estimate.bytes,
            source: estimate.source.clone(),
        });
        Ok(Reservation {
            id,
            bytes: estimate.bytes,
        })
    }

    fn release(&self, reservation: Reservation) {
        if let Ok(mut reserved) = self.reserved.lock() {
            *reserved = reserved.saturating_sub(reservation.bytes);
        }
        self.push(AdmissionEvent::Release {
            id: reservation.id,
            bytes: reservation.bytes,
        });
    }

    fn commit(&self, reservation: Reservation, key: &str) {
        if let Ok(mut reserved) = self.reserved.lock() {
            *reserved = reserved.saturating_sub(reservation.bytes);
        }
        if let Ok(mut committed) = self.committed.lock() {
            committed.insert(key.to_string(), reservation.bytes);
        }
        self.push(AdmissionEvent::Commit {
            id: reservation.id,
            bytes: reservation.bytes,
            key: key.to_string(),
        });
    }

    fn release_residency(&self, key: &str) {
        let bytes = self
            .committed
            .lock()
            .ok()
            .and_then(|mut c| c.remove(key))
            .unwrap_or(0);
        if bytes > 0 {
            self.push(AdmissionEvent::ReleaseResidency {
                key: key.to_string(),
                bytes,
            });
        }
    }

    fn take_events(&self) -> Vec<AdmissionEvent> {
        self.events.lock().map(|mut g| std::mem::take(&mut *g)).unwrap_or_default()
    }
}

/// Context / KV state request before prefill (US-027).
#[derive(Debug, Clone)]
pub struct ContextRequest {
    pub prompt_tokens: u32,
    pub max_new_tokens: u32,
}

impl ContextRequest {
    pub fn total_span(&self) -> u32 {
        self.prompt_tokens.saturating_add(self.max_new_tokens)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateAdmission {
    /// Request fits the configured context budget.
    Accept { n_ctx: u32 },
    /// Explicit shorter context offered — caller must re-request; never silent truncate.
    Negotiate { requested: u32, offered_n_ctx: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateEvent {
    Accept { prompt: u32, max_new: u32, n_ctx: u32 },
    Negotiate { requested: u32, offered: u32 },
    Reject { prompt: u32, max_new: u32, limit: u32 },
    Release { n_ctx: u32 },
}

/// Bounds per-session KV / context (US-027). Separate from weight [`MemoryBudget`].
pub trait InferenceStateBudget: Send + Sync {
    fn budget_source(&self) -> &str;
    fn max_context_tokens(&self) -> u32;
    /// When true, over-budget requests get [`StateAdmission::Negotiate`] instead of hard reject.
    fn allow_negotiate(&self) -> bool {
        false
    }
    fn admit(&self, req: &ContextRequest) -> Result<StateAdmission>;
    fn release(&self, admission: &StateAdmission);
    fn take_events(&self) -> Vec<StateEvent> {
        Vec::new()
    }
}

/// Always admits (default until a state budget is injected).
#[derive(Debug, Default)]
pub struct UnlimitedInferenceStateBudget;

impl InferenceStateBudget for UnlimitedInferenceStateBudget {
    fn budget_source(&self) -> &str {
        "unlimited_state"
    }

    fn max_context_tokens(&self) -> u32 {
        u32::MAX
    }

    fn admit(&self, _req: &ContextRequest) -> Result<StateAdmission> {
        // Callers clamp to CompleteOptions.context_tokens via resolve_context_n_ctx.
        Ok(StateAdmission::Accept { n_ctx: u32::MAX })
    }

    fn release(&self, _admission: &StateAdmission) {}
}

/// Resolve llama `n_ctx` from a budgeted admission (US-027).
///
/// [`StateAdmission::Negotiate`] becomes [`Error::StateBudgetNegotiate`] — never silent truncate.
pub fn resolve_context_n_ctx(
    budget: &dyn InferenceStateBudget,
    req: &ContextRequest,
    requested_n_ctx: u32,
) -> Result<(u32, StateAdmission)> {
    let admission = budget.admit(req)?;
    match &admission {
        StateAdmission::Accept { n_ctx } => {
            let effective = requested_n_ctx.min(*n_ctx).max(256);
            Ok((effective, admission))
        }
        StateAdmission::Negotiate {
            requested,
            offered_n_ctx,
        } => Err(Error::StateBudgetNegotiate {
            requested: *requested,
            offered_n_ctx: *offered_n_ctx,
            reason: format!(
                "re-request with context_tokens<={offered_n_ctx} (source={})",
                budget.budget_source()
            ),
        }),
    }
}

/// Deterministic context cap for CI (US-027).
pub struct FakeInferenceStateBudget {
    max_context_tokens: u32,
    allow_negotiate: bool,
    active: Mutex<u32>,
    events: Mutex<Vec<StateEvent>>,
}

impl FakeInferenceStateBudget {
    pub fn with_max_context(max_context_tokens: u32) -> Arc<Self> {
        Arc::new(Self {
            max_context_tokens,
            allow_negotiate: false,
            active: Mutex::new(0),
            events: Mutex::new(Vec::new()),
        })
    }

    pub fn with_negotiate(max_context_tokens: u32) -> Arc<Self> {
        Arc::new(Self {
            max_context_tokens,
            allow_negotiate: true,
            active: Mutex::new(0),
            events: Mutex::new(Vec::new()),
        })
    }

    fn push(&self, ev: StateEvent) {
        if let Ok(mut g) = self.events.lock() {
            g.push(ev);
        }
    }
}

impl InferenceStateBudget for FakeInferenceStateBudget {
    fn budget_source(&self) -> &str {
        "fake_state"
    }

    fn max_context_tokens(&self) -> u32 {
        self.max_context_tokens
    }

    fn allow_negotiate(&self) -> bool {
        self.allow_negotiate
    }

    fn admit(&self, req: &ContextRequest) -> Result<StateAdmission> {
        let span = req.total_span();
        if span <= self.max_context_tokens {
            if let Ok(mut active) = self.active.lock() {
                *active = active.saturating_add(self.max_context_tokens);
            }
            let adm = StateAdmission::Accept {
                n_ctx: self.max_context_tokens,
            };
            self.push(StateEvent::Accept {
                prompt: req.prompt_tokens,
                max_new: req.max_new_tokens,
                n_ctx: self.max_context_tokens,
            });
            return Ok(adm);
        }
        if self.allow_negotiate && req.prompt_tokens < self.max_context_tokens {
            let offered = self.max_context_tokens;
            self.push(StateEvent::Negotiate {
                requested: span,
                offered,
            });
            return Ok(StateAdmission::Negotiate {
                requested: span,
                offered_n_ctx: offered,
            });
        }
        self.push(StateEvent::Reject {
            prompt: req.prompt_tokens,
            max_new: req.max_new_tokens,
            limit: self.max_context_tokens,
        });
        Err(Error::StateBudgetRejected {
            prompt_tokens: req.prompt_tokens,
            max_new_tokens: req.max_new_tokens,
            limit_tokens: self.max_context_tokens,
            reason: format!(
                "context span {span} exceeds limit {} (source={})",
                self.max_context_tokens,
                self.budget_source()
            ),
        })
    }

    fn release(&self, admission: &StateAdmission) {
        if let StateAdmission::Accept { n_ctx } = admission {
            if let Ok(mut active) = self.active.lock() {
                *active = active.saturating_sub(*n_ctx);
            }
            self.push(StateEvent::Release { n_ctx: *n_ctx });
        }
    }

    fn take_events(&self) -> Vec<StateEvent> {
        self.events
            .lock()
            .map(|mut g| std::mem::take(&mut *g))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_rejects_when_over_capacity() {
        let b = FakeMemoryBudget::with_capacity(100);
        let r = b.try_reserve(&MemoryEstimate::exact(60, "a")).unwrap();
        b.commit(r, "a"); // path key
        let err = b.try_reserve(&MemoryEstimate::exact(50, "b")).unwrap_err();
        assert!(matches!(err, Error::AdmissionRejected { .. }));
        let ev = b.take_events();
        assert!(ev.iter().any(|e| matches!(e, AdmissionEvent::Reject { .. })));
    }

    #[test]
    fn fake_release_allows_retry() {
        let b = FakeMemoryBudget::with_capacity(100);
        let r = b.try_reserve(&MemoryEstimate::exact(80, "a")).unwrap();
        b.release(r);
        b.try_reserve(&MemoryEstimate::exact(80, "b")).unwrap();
    }

    #[test]
    fn state_budget_rejects_over_context() {
        let b = FakeInferenceStateBudget::with_max_context(128);
        let err = b
            .admit(&ContextRequest {
                prompt_tokens: 100,
                max_new_tokens: 50,
            })
            .unwrap_err();
        assert!(matches!(err, Error::StateBudgetRejected { .. }));
    }

    #[test]
    fn state_budget_negotiate_when_enabled() {
        let b = FakeInferenceStateBudget::with_negotiate(128);
        let adm = b
            .admit(&ContextRequest {
                prompt_tokens: 100,
                max_new_tokens: 50,
            })
            .unwrap();
        assert!(matches!(
            adm,
            StateAdmission::Negotiate {
                offered_n_ctx: 128,
                ..
            }
        ));
    }

    #[test]
    fn state_budget_accept_and_release() {
        let b = FakeInferenceStateBudget::with_max_context(128);
        let adm = b
            .admit(&ContextRequest {
                prompt_tokens: 32,
                max_new_tokens: 32,
            })
            .unwrap();
        b.release(&adm);
        let ev = b.take_events();
        assert!(ev.iter().any(|e| matches!(e, StateEvent::Accept { .. })));
        assert!(ev.iter().any(|e| matches!(e, StateEvent::Release { .. })));
    }
}
