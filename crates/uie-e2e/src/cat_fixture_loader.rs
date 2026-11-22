//! T2I stub that emits a known cat PNG tensor (US-012 e2e ground truth).

use std::path::{Path, PathBuf};
use std::sync::Arc;

use uie_core::backend::Backend;
use uie_core::error::{Error, Result};
use uie_core::model::{InferenceExecutor, ModelLoader};
use uie_stub_models::{ModalityKind, RuleStubPolicy, StubExecutor, StubMatcher};

use crate::spiro_image::load_png_nchw_rgb;

pub const CAT_STUB_NAME: &str = "diffusion_t2i_cat.stub";

#[derive(Debug)]
pub struct CatFixtureDiffusionLoader {
    cat_png: PathBuf,
}

impl CatFixtureDiffusionLoader {
    pub fn from_repo(repo: &Path) -> Self {
        Self {
            cat_png: repo.join("fixtures/images/cat_512.png"),
        }
    }
}

impl ModelLoader for CatFixtureDiffusionLoader {
    fn format_id(&self) -> &str {
        "stub_cat_fixture"
    }

    fn can_load(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n == CAT_STUB_NAME)
    }

    fn load(&self, path: &Path, backend: &dyn Backend) -> Result<Box<dyn InferenceExecutor>> {
        let _ = (path, backend);
        if !self.cat_png.is_file() {
            return Err(Error::ModelLoad(format!(
                "cat fixture missing: {}",
                self.cat_png.display()
            )));
        }
        let cat = load_png_nchw_rgb(&self.cat_png, 512, 512)?;
        let policy = RuleStubPolicy::new(ModalityKind::DiffusionTextToImage)
            .rule_tensors(StubMatcher::Always, vec![cat]);
        Ok(Box::new(StubExecutor::with_policy(
            ModalityKind::DiffusionTextToImage,
            Arc::new(policy),
        )))
    }
}
