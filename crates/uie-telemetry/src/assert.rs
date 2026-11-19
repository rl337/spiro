use crate::trace::ExecutionTrace;

pub fn assert_phases(trace: &ExecutionTrace, expected: &[&str]) {
    for name in expected {
        assert!(
            trace.phases.iter().any(|p| p.name == *name),
            "missing telemetry phase `{name}`; have: {:?}",
            trace
                .phases
                .iter()
                .map(|p| p.name.as_str())
                .collect::<Vec<_>>()
        );
    }
}
