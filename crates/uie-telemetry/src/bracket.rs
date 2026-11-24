//! Trustworthy A/B/A benchmark brackets (US-028).
//!
//! Extends US-023 metrics with validity gates — does not replace [`InferMetrics`].

use serde::{Deserialize, Serialize};

/// Frozen workload identity — only declared variables may change across a bracket.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkloadFingerprint {
    pub workload_id: String,
    pub model_hash: String,
    pub tokenizer_hash: String,
    pub backend_revision: String,
    pub generation_cap: u32,
    pub cold_warm: ColdWarm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColdWarm {
    Cold,
    Warm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrialRole {
    Control,
    Candidate,
}

/// One timed trial. Missing rates are [`None`] — never coerce to zero.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialSample {
    pub role: TrialRole,
    pub fingerprint: WorkloadFingerprint,
    /// Decode throughput (tok/s); `None` = unknown / not measured.
    pub decode_tokens_per_sec: Option<f64>,
    pub ok: bool,
    pub negative_control_failed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BracketVerdict {
    /// Candidate beat both controls beyond margin with valid samples.
    EligibleWin,
    /// Controls disagree / thermal drift — do not promote.
    DriftInconclusive,
    /// Missing samples, unknown counters, or failed requests.
    Incomplete { reason: String },
    /// Faster-but-wrong / over-budget / retrieval fail — retain data, do not promote.
    NegativeControlBlocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BracketResult {
    pub verdict: BracketVerdict,
    pub control_before: Option<f64>,
    pub candidate: Option<f64>,
    pub control_after: Option<f64>,
    pub margin_frac: f64,
}

/// Evaluate control → candidate → control (A/B/A).
///
/// `margin_frac` is a relative threshold (e.g. `0.03` = 3%). A win requires the candidate
/// to exceed **both** controls by more than this fraction. If the two controls disagree with
/// each other by more than the margin (relative to their mean), verdict is drift.
pub fn evaluate_aba(
    control_before: &TrialSample,
    candidate: &TrialSample,
    control_after: &TrialSample,
    margin_frac: f64,
) -> BracketResult {
    let fp0 = &control_before.fingerprint;
    if !fingerprints_comparable(fp0, &candidate.fingerprint)
        || !fingerprints_comparable(fp0, &control_after.fingerprint)
    {
        return BracketResult {
            verdict: BracketVerdict::Incomplete {
                reason: "workload fingerprint mismatch".into(),
            },
            control_before: control_before.decode_tokens_per_sec,
            candidate: candidate.decode_tokens_per_sec,
            control_after: control_after.decode_tokens_per_sec,
            margin_frac,
        };
    }

    if control_before.negative_control_failed
        || candidate.negative_control_failed
        || control_after.negative_control_failed
    {
        return BracketResult {
            verdict: BracketVerdict::NegativeControlBlocked,
            control_before: control_before.decode_tokens_per_sec,
            candidate: candidate.decode_tokens_per_sec,
            control_after: control_after.decode_tokens_per_sec,
            margin_frac,
        };
    }

    if !control_before.ok || !candidate.ok || !control_after.ok {
        return BracketResult {
            verdict: BracketVerdict::Incomplete {
                reason: "trial reported not ok".into(),
            },
            control_before: control_before.decode_tokens_per_sec,
            candidate: candidate.decode_tokens_per_sec,
            control_after: control_after.decode_tokens_per_sec,
            margin_frac,
        };
    }

    let (Some(a), Some(b), Some(c)) = (
        control_before.decode_tokens_per_sec,
        candidate.decode_tokens_per_sec,
        control_after.decode_tokens_per_sec,
    ) else {
        return BracketResult {
            verdict: BracketVerdict::Incomplete {
                reason: "decode_tokens_per_sec unknown (not zero)".into(),
            },
            control_before: control_before.decode_tokens_per_sec,
            candidate: candidate.decode_tokens_per_sec,
            control_after: control_after.decode_tokens_per_sec,
            margin_frac,
        };
    };

    let mean_ctrl = (a + c) / 2.0;
    if mean_ctrl > 0.0 {
        let ctrl_spread = (a - c).abs() / mean_ctrl;
        if ctrl_spread > margin_frac {
            return BracketResult {
                verdict: BracketVerdict::DriftInconclusive,
                control_before: Some(a),
                candidate: Some(b),
                control_after: Some(c),
                margin_frac,
            };
        }
    }

    let win_vs_a = b > a * (1.0 + margin_frac);
    let win_vs_c = b > c * (1.0 + margin_frac);
    let verdict = if win_vs_a && win_vs_c {
        BracketVerdict::EligibleWin
    } else {
        // Not a clear win — treat as inconclusive rather than inventing "loss".
        BracketVerdict::DriftInconclusive
    };

    BracketResult {
        verdict,
        control_before: Some(a),
        candidate: Some(b),
        control_after: Some(c),
        margin_frac,
    }
}

fn fingerprints_comparable(a: &WorkloadFingerprint, b: &WorkloadFingerprint) -> bool {
    a.workload_id == b.workload_id
        && a.model_hash == b.model_hash
        && a.tokenizer_hash == b.tokenizer_hash
        && a.backend_revision == b.backend_revision
        && a.generation_cap == b.generation_cap
    // cold_warm intentionally ignored for comparability of the frozen identity fields above
}

/// Token accounting: prefer server/tokenizer usage; never count transport chunks as tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenCount {
    Known(u32),
    Unknown,
}

/// `usage` from the server/tokenizer wins. Chunk count is ignored for promotion metrics.
pub fn tokens_from_usage(usage_completion_tokens: Option<u32>) -> TokenCount {
    match usage_completion_tokens {
        Some(n) => TokenCount::Known(n),
        None => TokenCount::Unknown,
    }
}

/// Demonstrates the anti-pattern guard: chunk cardinality must not become a token rate.
pub fn reject_chunk_count_as_tokens(chunk_count: u32, usage: Option<u32>) -> TokenCount {
    let _ = chunk_count; // deliberately unused
    tokens_from_usage(usage)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fp() -> WorkloadFingerprint {
        WorkloadFingerprint {
            workload_id: "w".into(),
            model_hash: "m".into(),
            tokenizer_hash: "t".into(),
            backend_revision: "b".into(),
            generation_cap: 64,
            cold_warm: ColdWarm::Warm,
        }
    }

    fn sample(role: TrialRole, rate: Option<f64>) -> TrialSample {
        TrialSample {
            role,
            fingerprint: fp(),
            decode_tokens_per_sec: rate,
            ok: true,
            negative_control_failed: false,
        }
    }

    #[test]
    fn drift_when_controls_disagree() {
        let r = evaluate_aba(
            &sample(TrialRole::Control, Some(100.0)),
            &sample(TrialRole::Candidate, Some(108.0)),
            &sample(TrialRole::Control, Some(110.0)),
            0.03,
        );
        assert_eq!(r.verdict, BracketVerdict::DriftInconclusive);
    }

    #[test]
    fn eligible_win_stable_controls() {
        let r = evaluate_aba(
            &sample(TrialRole::Control, Some(100.0)),
            &sample(TrialRole::Candidate, Some(120.0)),
            &sample(TrialRole::Control, Some(100.0)),
            0.03,
        );
        assert_eq!(r.verdict, BracketVerdict::EligibleWin);
    }

    #[test]
    fn unknown_rate_is_incomplete_not_zero() {
        let r = evaluate_aba(
            &sample(TrialRole::Control, Some(100.0)),
            &sample(TrialRole::Candidate, None),
            &sample(TrialRole::Control, Some(100.0)),
            0.03,
        );
        assert!(matches!(
            r.verdict,
            BracketVerdict::Incomplete { reason } if reason.contains("unknown")
        ));
    }

    #[test]
    fn negative_control_blocks() {
        let mut cand = sample(TrialRole::Candidate, Some(200.0));
        cand.negative_control_failed = true;
        let r = evaluate_aba(
            &sample(TrialRole::Control, Some(100.0)),
            &cand,
            &sample(TrialRole::Control, Some(100.0)),
            0.03,
        );
        assert_eq!(r.verdict, BracketVerdict::NegativeControlBlocked);
    }

    #[test]
    fn usage_not_chunk_count() {
        assert_eq!(
            reject_chunk_count_as_tokens(3, Some(9)),
            TokenCount::Known(9)
        );
        assert_eq!(reject_chunk_count_as_tokens(3, None), TokenCount::Unknown);
    }
}
