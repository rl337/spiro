//! Live-mode acceptance: real weights, registry roots, optional ONNX (feature `live-onnx`).
mod support;

use std::path::PathBuf;

use uie_catalog::{ModelCatalog, ModelRootsFile, ModelRootConfig, ModelRootKind};
#[test]
fn live_us_021_scan_configured_roots() {
    if support::skip_unless_live("live_us_021_scan_configured_roots") {
        return;
    }

    let roots_path = std::env::var("SPIRO_MODEL_ROOTS")
        .map(PathBuf::from)
        .unwrap_or_else(|_| support::repo_root().join("config/model_roots.example.yaml"));

    if !roots_path.is_file() {
        eprintln!("skip: no model roots file at {}", roots_path.display());
        return;
    }

    let catalog = ModelCatalog::load(&roots_path).expect("load roots yaml");
    catalog.registry().stats();
    let n = catalog.registry().entries().len();
    eprintln!(
        "live registry: {} entries from {} roots scanned",
        n,
        catalog.registry().stats().roots_scanned
    );
    // Laptop may have empty roots until paths are edited — do not hard-fail zero.
}

#[test]
fn live_us_021_scan_env_home_lmstudio() {
    if support::skip_unless_live("live_us_021_scan_env_home_lmstudio") {
        return;
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let lm = PathBuf::from(home).join(".lmstudio/models");
    if !lm.is_dir() {
        eprintln!("skip: {} not present", lm.display());
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
    eprintln!("live lmstudio gguf count: {}", ggufs.len());
    if ggufs.is_empty() {
        if std::env::var("UIE_LIVE_REQUIRE_LMSTUDIO").is_ok_and(|v| v == "1") {
            panic!("UIE_LIVE_REQUIRE_LMSTUDIO set but no .gguf under ~/.lmstudio/models");
        }
        eprintln!("note: no .gguf found (ok if LM Studio not installed on this host)");
        return;
    }
    eprintln!("live sample: {:?}", ggufs[0].path);
}

#[test]
fn live_us_007_load_path_from_env() {
    if support::skip_unless_live("live_us_007_load_path_from_env") {
        return;
    }
    let path = match std::env::var("UIE_LIVE_MODEL_PATH") {
        Ok(p) => PathBuf::from(p),
        Err(_) => {
            eprintln!("skip: set UIE_LIVE_MODEL_PATH to a .onnx, .gguf, or .stub on disk");
            return;
        }
    };
    assert!(
        path.is_file(),
        "UIE_LIVE_MODEL_PATH not found: {}",
        path.display()
    );

    use uie_core::backend::cpu::CpuBackend;
    use uie_core::InferenceRuntime;
    use uie_stub_models::StubModelLoader;

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
    eprintln!(
        "live loaded: {} ({})",
        model.metadata().name,
        model.metadata().format
    );
    assert!(!model.inputs().is_empty());
}

#[cfg(feature = "live-onnx")]
#[test]
fn live_us_009_mnist_onnx_infer() {
    if support::skip_unless_live("live_us_009_mnist_onnx_infer") {
        return;
    }

    let model_path = std::env::var("UIE_MNIST_MODEL")
        .map(PathBuf::from)
        .unwrap_or_else(|_| support::repo_root().join("models/mnist-8.onnx"));
    let fixture = std::env::var("UIE_MNIST_FIXTURE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| support::repo_root().join("models/fixtures/digit_one_f32.bin"));

    if !model_path.is_file() || !fixture.is_file() {
        eprintln!("skip: mnist model or fixture missing");
        return;
    }

    use uie_core::backend::cpu::CpuBackend;
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
            uie_core::telemetry::ExecutionRecorder::new("live-mnist"),
        )
        .unwrap();
    let outputs = outcome.outputs;
    let logits = outputs[0].as_f32_slice().unwrap();
    let digit = logits
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(d, _)| d)
        .unwrap();
    assert_eq!(digit, 1, "fixture is digit one");
}

#[cfg(feature = "gguf")]
#[test]
fn live_us_010_gguf_text_completion() {
    if support::skip_unless_live("live_us_010_gguf_text_completion") {
        return;
    }
    let model_path = match std::env::var("UIE_GGUF_MODEL").or_else(|_| std::env::var("UIE_LIVE_MODEL_PATH")) {
        Ok(p) => std::path::PathBuf::from(p),
        Err(_) => {
            eprintln!("skip: set UIE_GGUF_MODEL to a .gguf from LM Studio");
            return;
        }
    };
    if !model_path.is_file() {
        eprintln!("skip: model not found {}", model_path.display());
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
}

#[test]
fn live_us_022_byte_pipeline_workspace_onnx_or_stub() {
    if support::skip_unless_live("live_us_022_byte_pipeline_workspace_onnx_or_stub") {
        return;
    }
    let onnx = support::repo_root().join("models/mnist-8.onnx");
    let stub = support::stubs_dir().join("llm.stub");
    let hop_path = if onnx.is_file() {
        onnx
    } else if stub.is_file() {
        stub
    } else {
        eprintln!("skip: no onnx or stub for live pipeline");
        return;
    };

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
            eprintln!("skip: onnx pipeline needs --features live-onnx");
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
            return;
        }
    }

    use uie_stub_models::{sample_input, ModalityKind};
    let ins = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let out = runner.run_tensors("live-pipe", &[ins[0].clone()]).unwrap();
    assert!(!out.is_empty());
}