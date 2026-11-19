use std::path::Path;

use tracing::{debug, warn};
use walkdir::WalkDir;

use crate::comfy::classify_comfy_path;
use crate::lmstudio::classify_lmstudio_path;
use crate::registry::{ModelEntry, ModelFormat, ModelRole, ScanStats};
use crate::roots::{expand_home, ModelRootKind, ModelRootsFile};

pub fn scan_roots(roots: &ModelRootsFile) -> (Vec<ModelEntry>, ScanStats) {
    let mut entries = Vec::new();
    let mut stats = ScanStats::default();

    for root in &roots.roots {
        let path = expand_home(root.path.clone());
        if !path.is_dir() {
            warn!(root = %root.id, path = %path.display(), "model root missing, skipping");
            stats.roots_skipped += 1;
            continue;
        }
        stats.roots_scanned += 1;
        let before = entries.len();
        scan_one_root(&root.id, &path, root.kind, root.max_depth, &mut entries);
        stats.files_indexed += entries.len() - before;
        debug!(
            root = %root.id,
            added = entries.len() - before,
            "root scan complete"
        );
    }

    entries.sort_by(|a, b| a.id.cmp(&b.id));
    (entries, stats)
}

fn scan_one_root(
    root_id: &str,
    base: &Path,
    kind: ModelRootKind,
    max_depth: usize,
    out: &mut Vec<ModelEntry>,
) {
    for entry in WalkDir::new(base)
        .max_depth(max_depth)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(model) = classify_file(root_id, base, kind, path) {
            out.push(model);
        }
    }
}

fn classify_file(
    root_id: &str,
    base: &Path,
    kind: ModelRootKind,
    path: &Path,
) -> Option<ModelEntry> {
    let rel = path.strip_prefix(base).ok()?;
    let rel_str = rel.to_string_lossy().replace('\\', "/");

    match kind {
        ModelRootKind::LmStudio => classify_lmstudio_path(root_id, base, path, &rel_str),
        ModelRootKind::ComfyUi => classify_comfy_path(root_id, base, path, &rel_str),
        ModelRootKind::Generic => classify_generic(root_id, base, path, &rel_str),
    }
}

fn classify_generic(root_id: &str, _base: &Path, path: &Path, rel_str: &str) -> Option<ModelEntry> {
    let format = extension_format(path)?;
    let id = format!("{root_id}:{rel_str}");
    Some(ModelEntry {
        id,
        root_id: root_id.into(),
        path: path.to_path_buf(),
        format,
        role: ModelRole::Unknown,
        family: path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(str::to_string),
    })
}

pub(crate) fn extension_format(path: &Path) -> Option<ModelFormat> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    match ext.as_str() {
        "gguf" => Some(ModelFormat::Gguf),
        "onnx" => Some(ModelFormat::Onnx),
        "safetensors" => Some(ModelFormat::Safetensors),
        "stub" => Some(ModelFormat::Stub),
        "ckpt" | "pt" | "pth" | "bin" => Some(ModelFormat::Other),
        _ => None,
    }
}

pub(crate) fn id_from_rel(root_id: &str, rel_str: &str) -> String {
    format!("{root_id}:{rel_str}")
}
