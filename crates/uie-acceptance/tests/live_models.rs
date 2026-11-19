//! Live-mode acceptance: real weights, registry roots, optional ONNX (feature `live-onnx`).
mod support;

use std::path::PathBuf;

use uie_acceptance::{assert_infer_outcome_trace, assert_story_events, StoryEventLog};
use uie_catalog::{ModelCatalog, ModelRootsFile, ModelRootConfig, ModelRootKind};

fn skip_live(log: &mut StoryEventLog, test_name: &str) -> bool {
    if support::skip_unless_live(test_name) {
        log.record("live_skipped_ci");
        assert_story_events(log, &["live_skipped_ci"]);
        return true;
    }
    false
}

#[test]
fn live_us_021_scan_configured_roots() {
    let mut log = StoryEventLog::new("US-021");
    if skip_live(&mut log, "live_us_021_scan_configured_roots") {
        return;
    }

    let roots_path = std::env::var("SPIRO_MODEL_ROOTS")
        .map(PathBuf::from)
        .unwrap_or_else(|_| support::repo_root().join("config/model_roots.example.yaml"));

    if !roots_path.is_file() {
        log.record("live_skipped_no_roots_file");
        assert_story_events(&log, &["live_skipped_no_roots_file"]);
        return;
    }

    let catalog = ModelCatalog::load(&roots_path).expect("load roots yaml");
    let n = catalog.registry().entries().len();
    log.record("registry_live_scan");
    eprintln!(
        "live registry: {} entries from {} roots scanned",
        n,
        catalog.registry().stats().roots_scanned
    );
    assert_story_events(&log, &["registry_live_scan"]);
}

#[test]
fn live_us_021_scan_env_home_lmstudio() {
    let mut log = StoryEventLog::new("US-021");
    if skip_live(&mut log, "live_us_021_scan_env_home_lmstudio") {
        return;
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let lm = PathBuf::from(home).join(".lmstudio/models");
    if !lm.is_dir() {
        log.record("live_skipped_no_lmstudio_dir");
        assert_story_events(&log, &["live_skipped_no_lmstudio_dir"]);
        return;
    }
    let catalog = ModelCatalog::from_roots(ModelRootsFile {
        roots: vec![ModelRootConfig {
            id: "lmstudio".into(),
            path: lm,
            kind: ModelRootKind::LmStudio,
            max_depth: 8,
        }],
    });
    let ggufs: Vec<_> = catalog
        .registry()
        .entries()
        .iter()
        .filter(|e| e.path.extension().is_some_and(|x| x == "gguf"))
        .collect();
    log.record("registry_live_scan");
    eprintln!("live lmstudio gguf count: {}", ggufs.len());
    if ggufs.is_empty() {
        if std::env::var("UIE_LIVE_REQUIRE_LMSTUDIO").is_ok_and(|v| v == "1") {
            panic!("UIE_LIVE_REQUIRE_LMSTUDIO set but no .gguf under ~/.lmstudio/models");
        }
        log.record("live_lmstudio_empty");
        assert_story_events(&log, &["registry_live_scan", "live_lmstudio_empty"]);
        return;
    }
    eprintln!("live sample: {:?}", ggufs[0].path);
    assert_story_events(&log, &["registry_live_scan"]);
}

#[test]
fn live_us_007_load_path_from_env() {
    let mut log = StoryEventLog::new("US-007");
    if skip_live(&mut log, "live_us_007_load_path_from_env") {
        return;
    }
    let path = match std::env::var("UIE_LIVE_MODEL_PATH") {
        Ok(p) => PathBuf::from(p),
        Err(_) => {
            log.record("live_skipped_no_model_path");
            assert_story_events(&log, &["live_skipped_no_model_path"]);
            return;
        }
    };
    assert!(
        path.is_file(),
        "UIE_LIVE_MODEL_PATH not found: {}",
        path.display()
    );

    use uie_core::backend::cpu::CpuBackend;
    use uie_core::telemetry::ExecutionRecorder;
    use uie_core::InferenceRuntime;
    use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

    let mut rt = InferenceRuntime::new(Box::new(CpuBackend::new()))
        .with_loader(Box::new(StubModelLoader::default()));

    #[cfg(feature = "live-onnx")]
    {
        if path.extension().is_some_and(|e| e == "onnx") {
            rt = InferenceRuntime::new(Box::new(CpuBackend::new()))
                .with_loader(Box::new(uie_loader_onnx::OnnxModelLoader::new()));
        }
    }

    let model = rt.load_model(&path).expect("load live model");
    log.record("live_model_loaded");
    eprintln!(
        "live loaded: {} ({})",
        model.metadata().name,
        model.metadata().format
    );
    assert!(!model.inputs().is_empty());

    if path.extension().is_some_and(|e| e == "stub") {
        let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
        let mut session = rt.create_session();
        let outcome = rt
            .infer(
                model.as_ref(),
                &inputs,
                &mut session,
                ExecutionRecorder::new("live-us-007"),
            )
            .unwrap();
        assert_infer_outcome_trace(&outcome, "US-007");
        log.record("live_infer_trace");
        assert_story_events(&log, &["live_model_loaded", "live_infer_trace"]);
    } else {
        assert_story_events(&log, &["live_model_loaded"]);
    }
}

#[cfg(feature = "live-onnx")]
#[test]
fn live_us_009_mnist_onnx_infer() {
    let mut log = StoryEventLog::new("US-009");
    if skip_live(&mut log, "live_us_009_mnist_onnx_infer") {
        return;
    }

    let model_path = std::env::var("UIE_MNIST_MODEL")
        .map(PathBuf::from)
        .unwrap_or_else(|_| support::repo_root().join("models/mnist-8.onnx"));
    let fixture = std::env::var("UIE_MNIST_FIXTURE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| support::repo_root().join("models/fixtures/digit_one_f32.bin"));

    if !model_path.is_file() || !fixture.is_file() {
        log.record("live_skipped_missing_mnist_assets");
        assert_story_events(&log, &["live_skipped_missing_mnist_assets"]);
        return;
    }

    use uie_core::backend::cpu::CpuBackend;
    use uie_core::telemetry::ExecutionRecorder;
    use uie_core::tensor::{DataType, Shape, Tensor};
    use uie_core::InferenceRuntime;
    use uie_loader_onnx::OnnxModelLoader;

    const SHAPE: [usize; 4] = [1, 1, 28, 28];
    let bytes = std::fs::read(&fixture).unwrap();
    let input = Tensor::from_bytes(DataType::F32, Shape::new(SHAPE), bytes).unwrap();

    let rt = InferenceRuntime::new(Box::new(CpuBackend::new()))
        .with_loader(Box::new(OnnxModelLoader::new()));
    let model = rt.load_model(&model_path).unwrap();
    let mut session = rt.create_session();
    let outcome = rt
        .infer(
            model.as_ref(),
            &[input],
            &mut session,
            ExecutionRecorder::new("live-mnist"),
        )
        .unwrap();
    assert_infer_outcome_trace(&outcome, "US-009");
    let outputs = outcome.outputs;
    let logits = outputs[0].as_f32_slice().unwrap();
    let digit = logits
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(d, _)| d)
        .unwrap();
    assert_eq!(digit, 1, "fixture is digit one");
    log.record("live_mnist_infer_trace");
    assert_story_events(&log, &["live_mnist_infer_trace"]);
}

#[cfg(feature = "gguf")]
#[test]
fn live_us_010_gguf_text_completion() {
    let mut log = StoryEventLog::new("US-010");
    if skip_live(&mut log, "live_us_010_gguf_text_completion") {
        return;
    }
    let model_path = match std::env::var("UIE_GGUF_MODEL").or_else(|_| std::env::var("UIE_LIVE_MODEL_PATH")) {
        Ok(p) => std::path::PathBuf::from(p),
        Err(_) => {
            log.record("live_skipped_no_gguf_path");
            assert_story_events(&log, &["live_skipped_no_gguf_path"]);
            return;
        }
    };
    if !model_path.is_file() {
        log.record("live_skipped_gguf_missing");
        assert_story_events(&log, &["live_skipped_gguf_missing"]);
        return;
    }
    let prompt = std::env::var("UIE_LLM_PROMPT").unwrap_or_else(|_| "Say hi in one short sentence.".into());
    let opts = uie_loader_gguf::CompleteOptions {
        max_new_tokens: 24,
        ..Default::default()
    };
    let out = uie_loader_gguf::complete_from_prompt(&model_path, &prompt, &opts).expect("complete");
    eprintln!("completion: {}", out.text);
    assert!(!out.token_ids.is_empty());
    assert!(out.metrics.total_ms >= 0.0);
    log.record("gguf_completion_metrics");
    assert_story_events(&log, &["gguf_completion_metrics"]);
}

#[test]
fn live_us_022_byte_pipeline_workspace_onnx_or_stub() {
    let mut log = StoryEventLog::new("US-022");
    if skip_live(&mut log, "live_us_022_byte_pipeline_workspace_onnx_or_stub") {
        return;
    }
    let onnx = support::repo_root().join("models/mnist-8.onnx");
    let stubs = support::stubs_dir();
    let stub = stubs.join("llm.stub");
    let hop_path = if onnx.is_file() {
        onnx
    } else if stub.is_file() {
        stub
    } else {
        log.record("live_skipped_no_pipeline_model");
        assert_story_events(&log, &["live_skipped_no_pipeline_model"]);
        return;
    };

    use uie_acceptance::assert_ipc_infer_trace;
    use uie_ipc::{ByteChannel, IpcMessage, InProcessChannel, InferenceEngineHost, TensorPayload};
    use uie_pipeline::{ByteChannelPipelineRunner, PipelineHop};
    use uie_stub_models::StubModelLoader;

    let is_onnx = hop_path.extension().is_some_and(|e| e == "onnx");
    let hops = vec![
        PipelineHop {
            label: "a".into(),
            model_path: hop_path.clone(),
        },
        PipelineHop {
            label: "b".into(),
            model_path: hop_path.clone(),
        },
    ];

    let mut loaders: Vec<Box<dyn uie_core::model::ModelLoader>> =
        vec![Box::new(StubModelLoader::default())];
    #[cfg(feature = "live-onnx")]
    if is_onnx {
        loaders.push(Box::new(uie_loader_onnx::OnnxModelLoader::new()));
    }

    let mut runner = ByteChannelPipelineRunner::from_loaders(loaders, hops);

    if is_onnx {
        #[cfg(not(feature = "live-onnx"))]
        {
            log.record("live_skipped_onnx_feature");
            assert_story_events(&log, &["live_skipped_onnx_feature"]);
            return;
        }
        #[cfg(feature = "live-onnx")]
        {
            use uie_core::tensor::{DataType, Shape, Tensor};
            let fixture = support::repo_root().join("models/fixtures/digit_one_f32.bin");
            let bytes = std::fs::read(fixture).unwrap();
            let input = Tensor::from_bytes(DataType::F32, Shape::new([1, 1, 28, 28]), bytes).unwrap();
            let out = runner.run_tensors("live-pipe", &[input]).unwrap();
            assert!(!out.is_empty());
            log.record("live_pipeline_onnx_completed");
            assert_story_events(&log, &["live_pipeline_onnx_completed"]);
            return;
        }
    }

    use uie_stub_models::{sample_input, ModalityKind};
    let ins = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let out = runner.run_tensors("live-pipe", &[ins[0].clone()]).unwrap();
    assert!(!out.is_empty());

    let host = InferenceEngineHost::new(vec![Box::new(StubModelLoader::default())]);
    let mut ch = InProcessChannel::new(host);
    let payloads = vec![TensorPayload::from_tensor("input_ids", &ins[0]).unwrap()];
    let resp = ch
        .roundtrip(&IpcMessage::Infer {
            request_id: "live-us-022".into(),
            model_path: stubs.join("llm.stub").to_string_lossy().into(),
            inputs: payloads,
        })
        .unwrap();
    assert_ipc_infer_trace(&resp, "US-022");
    log.record("live_pipeline_hop_trace");
    assert_story_events(&log, &["live_pipeline_hop_trace"]);
}