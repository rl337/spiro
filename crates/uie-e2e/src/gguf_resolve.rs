use std::env;
use std::path::{Path, PathBuf};

use uie_catalog::{expand_home, ModelCatalog, ModelRootConfig, ModelRootKind, ModelRootsFile};
use uie_core::Result;

pub fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}

pub fn roots_file(repo_root: &Path) -> Result<ModelRootsFile> {
    let path = env::var("SPIRO_MODEL_ROOTS")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root.join("config/model_roots.yaml"));
    if path.is_file() {
        ModelRootsFile::load(&path)
    } else {
        Ok(default_roots())
    }
}

fn default_roots() -> ModelRootsFile {
    let home = env::var("HOME").unwrap_or_else(|_| ".".into());
    ModelRootsFile {
        roots: vec![ModelRootConfig {
            id: "lmstudio".into(),
            path: expand_home(format!("{home}/.lmstudio/models")),
            kind: ModelRootKind::LmStudio,
            max_depth: 8,
        }],
    }
}

#[derive(Debug, Clone)]
pub struct QwenVisionBundle {
    pub llm: PathBuf,
    pub mmproj: PathBuf,
}

/// Qwen2-VL ~2B (or similar): main `.gguf` plus sibling `*mmproj*.gguf` in the same directory.
pub fn find_qwen_vl_2b_bundle(catalog: &ModelCatalog) -> Result<QwenVisionBundle> {
    if let Ok(llm) = env::var("UIE_QWEN_VL_GGUF") {
        let mmproj = env::var("UIE_QWEN_VL_MMPROJ").map_err(|_| {
            uie_core::Error::ModelLoad(
                "UIE_QWEN_VL_GGUF set but UIE_QWEN_VL_MMPROJ is missing".into(),
            )
        })?;
        return Ok(QwenVisionBundle {
            llm: PathBuf::from(llm),
            mmproj: PathBuf::from(mmproj),
        });
    }

    let mut best: Option<(i32, QwenVisionBundle)> = None;
    for entry in catalog.registry().entries() {
        if entry.path.extension().is_none_or(|e| e != "gguf") {
            continue;
        }
        let name = entry.path.file_name().unwrap_or_default().to_string_lossy();
        if name.to_lowercase().contains("mmproj") {
            continue;
        }
        let hay = format!("{} {}", entry.id, entry.path.display()).to_lowercase();
        if !hay.contains("qwen") || !hay.contains("2b") {
            continue;
        }
        let Some(mmproj) = mmproj_beside_llm(&entry.path) else {
            continue;
        };
        let mut score = 0;
        if hay.contains("vl") || hay.contains("vision") {
            score += 10;
        }
        if hay.contains("instruct") {
            score += 2;
        }
        let bundle = QwenVisionBundle {
            llm: entry.path.clone(),
            mmproj,
        };
        if best.as_ref().is_none_or(|(s, _)| score > *s) {
            best = Some((score, bundle));
        }
    }

    best.map(|(_, b)| b).ok_or_else(|| {
        uie_core::Error::ModelLoad(
            "no Qwen2-VL ~2B pair (.gguf + mmproj) under SPIRO_MODEL_ROOTS — \
             install Qwen2-VL-2B-Instruct in LM Studio or set UIE_QWEN_VL_GGUF / UIE_QWEN_VL_MMPROJ"
                .into(),
        )
    })
}

fn mmproj_beside_llm(llm: &Path) -> Option<PathBuf> {
    let dir = llm.parent()?;
    let llm_stem = llm.file_stem()?.to_string_lossy().to_lowercase();
    let mut fallback = None;
    for ent in std::fs::read_dir(dir).ok()? {
        let path = ent.ok()?.path();
        if path.extension().is_none_or(|e| e != "gguf") {
            continue;
        }
        let fname = path.file_name()?.to_string_lossy().to_lowercase();
        if !fname.contains("mmproj") {
            continue;
        }
        if llm_stem
            .split(&['-', '_'][..])
            .filter(|p| p.len() > 2)
            .any(|part| fname.contains(part))
        {
            return Some(path);
        }
        if fname.contains("qwen") {
            fallback = Some(path);
        }
    }
    fallback
}

pub fn find_qwen3_2b_path(catalog: &ModelCatalog) -> Result<PathBuf> {
    let needles = ["qwen", "2b"];
    for entry in catalog.registry().entries() {
        if entry.path.extension().is_some_and(|e| e == "gguf") {
            let hay = format!("{} {}", entry.id, entry.path.display()).to_lowercase();
            if needles.iter().all(|n| hay.contains(n)) {
                eprintln!("matched registry id: {}", entry.id);
                return Ok(entry.path.clone());
            }
        }
    }
    Err(uie_core::Error::ModelLoad(
        "no Qwen3 ~2B .gguf under SPIRO_MODEL_ROOTS — see docs/runbooks/qwen3-2b-e2e.md".into(),
    ))
}
