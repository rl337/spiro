use std::path::{Path, PathBuf};

use crate::registry::{ModelEntry, ModelFormat, ModelRole};
use crate::scan::{extension_format, id_from_rel};

/// LM Studio layout: `publisher/repo/file.gguf`
pub fn classify_lmstudio_path(
    root_id: &str,
    _base: &Path,
    path: &Path,
    rel_str: &str,
) -> Option<ModelEntry> {
    if extension_format(path)? != ModelFormat::Gguf {
        return None;
    }
    let parts: Vec<_> = rel_str.split('/').collect();
    let family = parts.first().map(|s| (*s).to_string());
    Some(ModelEntry {
        id: id_from_rel(root_id, rel_str),
        root_id: root_id.into(),
        path: path.to_path_buf(),
        format: ModelFormat::Gguf,
        role: ModelRole::Llm,
        family,
    })
}