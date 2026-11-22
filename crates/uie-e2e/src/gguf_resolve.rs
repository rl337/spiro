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

/// LM Studio **VL** weights: main `.gguf` plus sibling `*mmproj*.gguf`.
///
/// Vision is detected by the **mmproj file**, not by "vl" in the filename (many LM Studio
/// trees use `model.gguf` + `mmproj-….gguf` for Qwen3-VL ~2B).
/// Every registry GGUF with `qwen` in the id and a sibling mmproj (for `list-vl-bundles`).
pub fn list_qwen_vl_bundles(catalog: &ModelCatalog) -> Vec<QwenVisionBundle> {
    let mut out = Vec::new();
    for entry in catalog.registry().entries() {
        if entry.path.extension().is_none_or(|e| e != "gguf") {
            continue;
        }
        let name = entry.path.file_name().unwrap_or_default().to_string_lossy();
        if name.to_lowercase().contains("mmproj") {
            continue;
        }
        let hay = format!("{} {}", entry.id, entry.path.display()).to_lowercase();
        if !hay.contains("qwen") {
            continue;
        }
        let Some(mmproj) = mmproj_beside_llm(&entry.path) else {
            continue;
        };
        out.push(QwenVisionBundle {
            llm: entry.path.clone(),
            mmproj,
        });
    }
    out.sort_by(|a, b| a.llm.to_string_lossy().cmp(&b.llm.to_string_lossy()));
    out
}

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

    if let Ok(id) = env::var("UIE_QWEN_VL_REGISTRY_ID") {
        let entry = catalog
            .registry()
            .entries()
            .iter()
            .find(|e| e.id == id)
            .ok_or_else(|| {
                uie_core::Error::ModelLoad(format!("UIE_QWEN_VL_REGISTRY_ID not found: {id}"))
            })?;
        let mmproj = mmproj_beside_llm(&entry.path).ok_or_else(|| {
            uie_core::Error::ModelLoad(format!(
                "no mmproj .gguf beside registry entry {} ({})",
                id,
                entry.path.display()
            ))
        })?;
        return Ok(QwenVisionBundle {
            llm: entry.path.clone(),
            mmproj,
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
        if !hay.contains("qwen") {
            continue;
        }
        let Some(mmproj) = mmproj_beside_llm(&entry.path) else {
            continue;
        };
        let mut score = 5; // qwen + mmproj ⇒ treat as VL (LM Studio layout)
        if hay.contains("2b") || hay.contains("2.0b") {
            score += 8;
        }
        if hay.contains("0.6b") || hay.contains("0_6b") || hay.contains("0-6b") {
            score += 6;
        }
        if hay.contains("1.7b") {
            score += 4;
        }
        if hay.contains("visual") || hay.contains("vl") || hay.contains("vision") {
            score += 6;
        }
        if hay.contains("instruct") || hay.contains("chat") {
            score += 2;
        }
        if hay.contains("mmproj") {
            score -= 100;
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
            "no Qwen VL pair (.gguf + mmproj in the same folder) under SPIRO_MODEL_ROOTS — \
             in LM Studio, open the Qwen ~2B **VL** model folder (not text-only) or set \
             UIE_QWEN_VL_REGISTRY_ID / UIE_QWEN_VL_GGUF + UIE_QWEN_VL_MMPROJ"
                .into(),
        )
    })
}

/// Pair an LLM `.gguf` with an `*mmproj*.gguf` in the same directory (LM Studio layout).
pub fn mmproj_beside_llm(llm: &Path) -> Option<PathBuf> {
    let dir = llm.parent()?;
    let llm_stem = llm.file_stem()?.to_string_lossy().to_lowercase();

    let mut mmprojs: Vec<PathBuf> = Vec::new();
    for ent in std::fs::read_dir(dir).ok()? {
        let path = ent.ok()?.path();
        if path.extension().is_none_or(|e| e != "gguf") {
            continue;
        }
        let fname = path.file_name()?.to_string_lossy().to_lowercase();
        if fname.contains("mmproj") {
            mmprojs.push(path);
        }
    }

    if mmprojs.is_empty() {
        return None;
    }
    if mmprojs.len() == 1 {
        return Some(mmprojs.pop()?);
    }

    for path in &mmprojs {
        let fname = path.file_name()?.to_string_lossy().to_lowercase();
        if llm_stem
            .split(&['-', '_', '.'][..])
            .filter(|p| p.len() > 2 && *p != "model" && *p != "gguf")
            .any(|part| fname.contains(part))
        {
            return Some(path.clone());
        }
    }

    mmprojs
        .into_iter()
        .find(|p| {
            p.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase()
                .contains("qwen")
        })
        .or_else(|| {
            // Last resort: same folder, multiple mmprojs — cannot disambiguate.
            None
        })
}

pub fn find_qwen3_2b_path(catalog: &ModelCatalog) -> Result<PathBuf> {
    let needles = ["qwen", "2b"];
    for entry in catalog.registry().entries() {
        if entry.path.extension().is_some_and(|e| e == "gguf") {
            let name = entry.path.file_name().unwrap_or_default().to_string_lossy();
            if name.to_lowercase().contains("mmproj") {
                continue;
            }
            if mmproj_beside_llm(&entry.path).is_some() {
                continue;
            }
            let hay = format!("{} {}", entry.id, entry.path.display()).to_lowercase();
            if needles.iter().all(|n| hay.contains(n)) {
                eprintln!("matched registry id: {}", entry.id);
                return Ok(entry.path.clone());
            }
        }
    }
    Err(uie_core::Error::ModelLoad(
        "no text-only Qwen3 ~2B .gguf under SPIRO_MODEL_ROOTS — see docs/runbooks/qwen3-2b-e2e.md"
            .into(),
    ))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::mmproj_beside_llm;

    #[test]
    fn pairs_lmstudio_model_dot_gguf_with_sole_mmproj() {
        let dir = tempfile::tempdir().unwrap();
        let llm = dir.path().join("model.gguf");
        let mm = dir.path().join("mmproj-model-f16.gguf");
        fs::write(&llm, b"").unwrap();
        fs::write(&mm, b"").unwrap();
        assert_eq!(mmproj_beside_llm(&llm), Some(mm));
    }

    #[test]
    fn pairs_by_shared_stem_tokens() {
        let dir = tempfile::tempdir().unwrap();
        let llm = dir.path().join("Qwen3-VL-2B-Instruct-Q4_K_M.gguf");
        let mm = dir.path().join("mmproj-Qwen3-VL-2B-Instruct-f16.gguf");
        fs::write(&llm, b"").unwrap();
        fs::write(&mm, b"").unwrap();
        assert_eq!(mmproj_beside_llm(&llm), Some(mm));
    }
}
