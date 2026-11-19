//! Lightweight **event capture** for acceptance tests that do not run a full infer trace.
//!
//! Use alongside [`crate::story_assert`] (telemetry / IPC traces) so every US-0NN test
//! asserts observability relevant to the story.

use std::collections::HashSet;

/// Append-only log of named events for a user story acceptance test.
#[derive(Debug, Clone)]
pub struct StoryEventLog {
    story_id: String,
    events: Vec<String>,
}

impl StoryEventLog {
    pub fn new(story_id: impl Into<String>) -> Self {
        Self {
            story_id: story_id.into(),
            events: Vec::new(),
        }
    }

    pub fn story_id(&self) -> &str {
        &self.story_id
    }

    pub fn record(&mut self, event: impl Into<String>) {
        self.events.push(event.into());
    }

    pub fn events(&self) -> &[String] {
        &self.events
    }

    /// Fails the test if any `required` event was not recorded.
    pub fn assert_events(&self, required: &[&str]) {
        let seen: HashSet<&str> = self.events.iter().map(|s| s.as_str()).collect();
        for name in required {
            assert!(
                seen.contains(name),
                "{}: missing story event `{name}` (have {:?})",
                self.story_id,
                self.events
            );
        }
    }
}

/// Shorthand for [`StoryEventLog::assert_events`].
pub fn assert_story_events(log: &StoryEventLog, required: &[&str]) {
    log.assert_events(required);
}