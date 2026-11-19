use std::env;
use std::path::PathBuf;

use uie_catalog::{expand_home, ModelCatalog, ModelRole, ModelRootsFile};
use uie_core::backend::cpu::CpuBackend;
use uie_core::tensor::Tensor;
use uie_core::InferenceRuntime;
use uie_loader_gguf::{complete_gguf_file, CompleteOptions, GgufModelLoader};

fn main() -> uie_core::Result<()> {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 && (args[1] == "list" || args[1] == "list-gguf") {
        return list_gguf();
    }

    let prompt = flag_value(&args, "--prompt")
        .or_else(|| env::var("UIE_LLM_PROMPT").ok())
        .unwrap_or_else(|| {
            eprintln!("usage: uie-llm complete --prompt \"...\" [--model PATH | --registry-id ID]");
            eprintln!("       uie-llm list-gguf");
            std::process::exit(2);
        });

    let max_new: usize = flag_value(&args, "--max-new")
        .and_then(|s| s.parse().ok())
        .or_else(|| env::var("UIE_LLM_MAX_NEW").ok()?.parse().ok())
        .unwrap_or(48);

    let model_path = resolve_model_path(&args)?;
    eprintln!("model: {}", model_path.display());

    let opts = CompleteOptions {
        max_new_tokens: max_new as u32,
        ..CompleteOptions::default()
    };

    if env::var("UIE_LLM_USE_RUNTIME").is_ok_and(|v| v == "1") {
        let rt = InferenceRuntime::new(Box::new(CpuBackend::new())).with_loader(Box::new(
            GgufModelLoader::new().with_max_new_tokens(max_new),
        ));
        let model = rt.load_model(&model_path)?;
        let prompt_t = uie_loader_gguf::prompt_tensor(&prompt, 4096)?;
        let mut session = rt.create_session();
        let outcome = rt.infer(
            model.as_ref(),
            &[prompt_t],
            &mut session,
            uie_core::telemetry::ExecutionRecorder::new("uie-llm-runtime"),
        )?;
        print_completion(&outcome.outputs);
        return Ok(());
    }

    let result = complete_gguf_file(&model_path, &prompt, &opts)?;
    println!("{}", result.text);
    eprintln!(
        "telemetry: total_ms={:.2} ttft_ms={:?} decode_tps={:?} cpu_ms={:.2} gpu_ms={:?}",
        result.metrics.total_ms,
        result.metrics.ttft_ms,
        result.metrics.decode_tokens_per_sec,
        result.metrics.cpu_time_ms,
        result.metrics.gpu_time_ms,
    );
    Ok(())
}

fn list_gguf() -> uie_core::Result<()> {
    let roots = roots_file()?;
    let catalog = ModelCatalog::from_roots(roots);
    let llms: Vec<_> = catalog
        .registry()
        .entries()
        .iter()
        .filter(|e| e.role == ModelRole::Llm)
        .collect();
    if llms.is_empty() {
        eprintln!("no .gguf models found — edit config/model_roots.yaml (lmstudio path)");
        return Ok(());
    }
    for e in llms {
        println!("{}\t{}", e.id, e.path.display());
    }
    Ok(())
}

fn resolve_model_path(args: &[String]) -> uie_core::Result<PathBuf> {
    if let Some(p) = flag_value(args, "--model").map(PathBuf::from) {
        return Ok(expand_home(p));
    }
    if let Ok(p) = env::var("UIE_GGUF_MODEL") {
        return Ok(expand_home(PathBuf::from(p)));
    }
    if let Some(id) = flag_value(args, "--registry-id") {
        let catalog = ModelCatalog::from_roots(roots_file()?);
        let entry = catalog
            .registry()
            .get(&id)
            .ok_or_else(|| uie_core::Error::ModelLoad(format!("registry id not found: {id}")))?;
        return Ok(entry.path.clone());
    }
    if let Ok(p) = env::var("UIE_LIVE_MODEL_PATH") {
        return Ok(expand_home(PathBuf::from(p)));
    }
    Err(uie_core::Error::ModelLoad(
        "set --model, UIE_GGUF_MODEL, --registry-id, or UIE_LIVE_MODEL_PATH".into(),
    ))
}

fn roots_file() -> uie_core::Result<ModelRootsFile> {
    let path = env::var("SPIRO_MODEL_ROOTS")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root().join("config/model_roots.yaml"));
    if path.is_file() {
        ModelRootsFile::load(&path)
    } else {
        Ok(default_roots())
    }
}

fn default_roots() -> ModelRootsFile {
    let home = env::var("HOME").unwrap_or_else(|_| ".".into());
    ModelRootsFile {
        roots: vec![uie_catalog::ModelRootConfig {
            id: "lmstudio".into(),
            path: expand_home(format!("{home}/.lmstudio/models")),
            kind: uie_catalog::ModelRootKind::LmStudio,
            max_depth: 8,
        }],
    }
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from("."))
}

fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}

fn print_completion(outs: &[Tensor]) {
    if let Some(t) = outs.first() {
        let bytes = t.as_bytes();
        let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        if let Ok(s) = std::str::from_utf8(&bytes[..end]) {
            println!("{s}");
        }
    }
}
