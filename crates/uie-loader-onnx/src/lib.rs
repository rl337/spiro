//! ONNX [`ModelLoader`] implementation backed by [tract](https://github.com/snipsco/tract) on CPU.

mod convert;
mod model;

pub use model::OnnxModel;
pub use model::OnnxModelLoader;
