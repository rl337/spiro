//! E2E harness: command-pattern tests, glob selection, shared context.

use std::fmt;
use std::path::PathBuf;

use glob::Pattern;

#[derive(Debug, thiserror::Error)]
pub enum E2eError {
    #[error("{0}")]
    Failed(String),
    #[error("no E2E test matches pattern `{0}`")]
    NoMatch(String),
}

/// Shared instrumentation and assertions for a single test run.
#[cfg_attr(not(feature = "gguf"), allow(dead_code))]
#[derive(Debug, Default)]
pub struct E2eContext {
    pub repo_root: PathBuf,
    events: Vec<String>,
    phases: Vec<String>,
}

#[cfg_attr(not(feature = "gguf"), allow(dead_code))]
impl E2eContext {
    pub fn new(repo_root: PathBuf) -> Self {
        Self {
            repo_root,
            ..Default::default()
        }
    }

    pub fn record(&mut self, event: impl Into<String>) {
        let e = event.into();
        self.events.push(e.clone());
        eprintln!("[e2e event] {e}");
    }

    pub fn phase(&mut self, name: impl Into<String>) {
        let p = name.into();
        self.phases.push(p.clone());
        eprintln!("[e2e phase] {p}");
    }

    pub fn assert(&self, cond: bool, msg: impl fmt::Display) -> Result<(), E2eError> {
        if cond {
            Ok(())
        } else {
            Err(E2eError::Failed(msg.to_string()))
        }
    }

    pub fn events(&self) -> &[String] {
        &self.events
    }
}

/// Command object: one end-to-end scenario (name + description + executable `run`).
pub trait E2eTest: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn run(&self, ctx: &mut E2eContext, args: &[String]) -> Result<(), E2eError>;
}

pub fn matches_pattern(pattern: &str, name: &str) -> bool {
    if pattern.contains('*') || pattern.contains('?') {
        Pattern::new(pattern)
            .map(|p| p.matches(name))
            .unwrap_or(false)
    } else {
        pattern == name
    }
}

pub fn select_tests<'a>(
    tests: &'a [Box<dyn E2eTest>],
    pattern: &str,
) -> Result<Vec<&'a dyn E2eTest>, E2eError> {
    let matched: Vec<&dyn E2eTest> = tests
        .iter()
        .filter(|t| matches_pattern(pattern, t.name()))
        .map(|b| b.as_ref())
        .collect();
    if matched.is_empty() {
        return Err(E2eError::NoMatch(pattern.to_string()));
    }
    Ok(matched)
}

pub fn run_selected(
    tests: &[&dyn E2eTest],
    ctx: &mut E2eContext,
    args: &[String],
) -> Result<(), E2eError> {
    let mut failures = Vec::new();
    for t in tests {
        eprintln!("=== E2E {} ===", t.name());
        eprintln!("{}", t.description());
        if let Err(e) = t.run(ctx, args) {
            failures.push(format!("{}: {}", t.name(), e));
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(E2eError::Failed(failures.join("; ")))
    }
}
