use std::sync::Arc;

use uie_core::error::Result;
use uie_core::tensor::Tensor;

use crate::infer_blank::infer_blank;
use crate::modality::ModalityKind;

/// Extend stub behavior without editing blank-output logic (used by tests and harnesses).
pub trait StubPolicy: Send + Sync {
    fn infer(&self, kind: ModalityKind, inputs: &[Tensor]) -> Result<Vec<Tensor>>;
}

/// Production default: validated inputs → canonical blank tensors.
#[derive(Debug, Default, Clone, Copy)]
pub struct BlankPolicy;

impl StubPolicy for BlankPolicy {
    fn infer(&self, kind: ModalityKind, inputs: &[Tensor]) -> Result<Vec<Tensor>> {
        infer_blank(kind, inputs)
    }
}

/// Ordered rules; first match wins, otherwise falls back to [`BlankPolicy`].
pub struct RuleStubPolicy {
    kind: ModalityKind,
    rules: Vec<StubRule>,
}

pub type StubRespond = Box<dyn Fn(&[Tensor]) -> Result<Vec<Tensor>> + Send + Sync>;

pub struct StubRule {
    pub matcher: StubMatcher,
    pub respond: StubRespond,
}

#[derive(Clone, Debug)]
pub enum StubMatcher {
    /// First `input_ids` / token tensor starts with these ids (length may differ).
    FirstInputTokenPrefix(Vec<i32>),
    /// First f32 tensor's first values match (e.g. audio marker sample).
    FirstInputF32Prefix(Vec<f32>),
    /// First pixel (NCHW) equals value at index 0 after batch/channel planes.
    FirstPixelF32(f32),
    /// Second input token prefix (VLM prompt).
    SecondInputTokenPrefix(Vec<i32>),
    Always,
}

impl RuleStubPolicy {
    pub fn new(kind: ModalityKind) -> Self {
        Self {
            kind,
            rules: Vec::new(),
        }
    }

    pub fn rule(
        mut self,
        matcher: StubMatcher,
        respond: impl Fn(&[Tensor]) -> Result<Vec<Tensor>> + Send + Sync + 'static,
    ) -> Self {
        self.rules.push(StubRule {
            matcher,
            respond: Box::new(respond),
        });
        self
    }

    pub fn rule_tensors(mut self, matcher: StubMatcher, outputs: Vec<Tensor>) -> Self {
        self.rules.push(StubRule {
            matcher,
            respond: Box::new(move |_| Ok(outputs.clone())),
        });
        self
    }
}

impl StubPolicy for RuleStubPolicy {
    fn infer(&self, kind: ModalityKind, inputs: &[Tensor]) -> Result<Vec<Tensor>> {
        if kind != self.kind {
            return infer_blank(kind, inputs);
        }
        for rule in &self.rules {
            if rule.matcher.matches(inputs)? {
                return (rule.respond)(inputs);
            }
        }
        infer_blank(kind, inputs)
    }
}

impl StubMatcher {
    pub fn matches(&self, inputs: &[Tensor]) -> Result<bool> {
        match self {
            StubMatcher::Always => Ok(true),
            StubMatcher::FirstInputTokenPrefix(prefix) => {
                let t = inputs.first().ok_or_else(|| {
                    uie_core::error::Error::InvalidShape("matcher: missing input 0".into())
                })?;
                let ids = t.as_i32_slice()?;
                Ok(ids.len() >= prefix.len() && ids[..prefix.len()] == **prefix)
            }
            StubMatcher::SecondInputTokenPrefix(prefix) => {
                let t = inputs.get(1).ok_or_else(|| {
                    uie_core::error::Error::InvalidShape("matcher: missing input 1".into())
                })?;
                let ids = t.as_i32_slice()?;
                Ok(ids.len() >= prefix.len() && ids[..prefix.len()] == **prefix)
            }
            StubMatcher::FirstInputF32Prefix(prefix) => {
                let t = inputs.first().ok_or_else(|| {
                    uie_core::error::Error::InvalidShape("matcher: missing input 0".into())
                })?;
                let slice = t.as_f32_slice()?;
                Ok(slice.len() >= prefix.len() && slice[..prefix.len()] == **prefix)
            }
            StubMatcher::FirstPixelF32(v) => {
                let t = inputs.first().ok_or_else(|| {
                    uie_core::error::Error::InvalidShape("matcher: missing input 0".into())
                })?;
                let slice = t.as_f32_slice()?;
                Ok(slice.first() == Some(v))
            }
        }
    }
}

/// Run inference through a custom policy (test harness helper).
pub fn infer_with_policy(
    policy: Arc<dyn StubPolicy>,
    kind: ModalityKind,
    inputs: &[Tensor],
) -> Result<Vec<Tensor>> {
    policy.infer(kind, inputs)
}
