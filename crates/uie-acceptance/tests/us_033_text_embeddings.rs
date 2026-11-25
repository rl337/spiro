//! US-033: in-process text embeddings (stub + telemetry).

use uie_acceptance::composition::injector;
use uie_embeddings::{
    assert_embed_telemetry, StubTextEmbedder, TextEmbedder, STUB_DIMENSIONS, STUB_MODEL_ID,
};
use uie_telemetry::ExecutionRecorder;

#[test]
fn us_033_stub_embed_batch_and_telemetry() {
    let embedder = injector().text_embedder();
    assert_eq!(embedder.model_id(), STUB_MODEL_ID);
    assert_eq!(embedder.dimensions(), STUB_DIMENSIONS);

    let texts = vec![
        "weirdness hub chunk one".into(),
        "second piece of prose".into(),
    ];
    let mut recorder = ExecutionRecorder::new("us-033");
    let vectors = embedder.embed(&texts, &mut recorder).unwrap();
    let trace = recorder.finish();

    assert_eq!(vectors.len(), 2);
    assert_eq!(vectors[0].len(), STUB_DIMENSIONS);
    assert_eq!(vectors[1].len(), STUB_DIMENSIONS);
    for v in &vectors {
        let n: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((n - 1.0).abs() < 1e-5, "vector must be L2-normalized");
    }
    assert_ne!(vectors[0], vectors[1]);
    assert_embed_telemetry(&trace);
}

#[test]
fn us_033_empty_batch_is_empty() {
    let stub = StubTextEmbedder::with_defaults();
    let mut recorder = ExecutionRecorder::new("us-033-empty");
    let out = stub.embed(&[], &mut recorder).unwrap();
    assert!(out.is_empty());
    assert_embed_telemetry(&recorder.finish());
}

#[test]
fn us_033_batch_over_64_fails_totally() {
    let stub = StubTextEmbedder::with_defaults();
    let texts: Vec<String> = (0..65).map(|i| format!("row-{i}")).collect();
    let mut recorder = ExecutionRecorder::new("us-033-overflow");
    assert!(stub.embed(&texts, &mut recorder).is_err());
}
