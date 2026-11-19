use serde::{Deserialize, Serialize};
use uie_core::tensor::{DataType, Shape, Tensor};

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IpcMessage {
    Hello { protocol_version: u32 },
    HelloAck {
        protocol_version: u32,
        engine: String,
    },
    Infer {
        request_id: String,
        model_path: String,
        inputs: Vec<TensorPayload>,
    },
    InferResult {
        request_id: String,
        outputs: Vec<TensorPayload>,
    },
    Error { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferRequest {
    pub request_id: String,
    pub model_path: String,
    pub inputs: Vec<TensorPayload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferResponse {
    pub request_id: String,
    pub outputs: Vec<TensorPayload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorPayload {
    pub name: String,
    pub dtype: String,
    pub shape: Vec<usize>,
    pub data: Vec<u8>,
}

impl From<InferRequest> for IpcMessage {
    fn from(r: InferRequest) -> Self {
        IpcMessage::Infer {
            request_id: r.request_id,
            model_path: r.model_path,
            inputs: r.inputs,
        }
    }
}

impl TryFrom<IpcMessage> for InferRequest {
    type Error = uie_core::Error;

    fn try_from(msg: IpcMessage) -> uie_core::Result<Self> {
        match msg {
            IpcMessage::Infer {
                request_id,
                model_path,
                inputs,
            } => Ok(Self {
                request_id,
                model_path,
                inputs,
            }),
            other => Err(uie_core::Error::Runtime(format!("not an infer request: {other:?}"))),
        }
    }
}

impl TensorPayload {
    pub fn from_tensor(name: impl Into<String>, tensor: &Tensor) -> uie_core::Result<Self> {
        Ok(Self {
            name: name.into(),
            dtype: dtype_name(tensor.dtype()),
            shape: tensor.shape().dims().to_vec(),
            data: tensor.as_bytes().to_vec(),
        })
    }

    pub fn to_tensor(&self) -> uie_core::Result<Tensor> {
        let dtype = parse_dtype(&self.dtype)?;
        Tensor::from_bytes(dtype, Shape::new(self.shape.clone()), self.data.clone())
    }
}

fn dtype_name(d: DataType) -> String {
    match d {
        DataType::F32 => "f32",
        DataType::F64 => "f64",
        DataType::I32 => "i32",
        DataType::U8 => "u8",
    }
    .into()
}

fn parse_dtype(s: &str) -> uie_core::Result<DataType> {
    match s {
        "f32" => Ok(DataType::F32),
        "f64" => Ok(DataType::F64),
        "i32" => Ok(DataType::I32),
        "u8" => Ok(DataType::U8),
        other => Err(uie_core::Error::InvalidShape(format!("unknown dtype `{other}`"))),
    }
}