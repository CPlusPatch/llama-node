use llama_sys::llama_context_params;
use napi::bindgen_prelude::*;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(Clone, Debug)]
pub struct InferenceToken {
    pub token: String,
    pub completed: bool,
}

#[napi(string_enum)]
pub enum InferenceResultType {
    Error,
    Data,
    End,
}

#[napi(object)]
pub struct InferenceResult {
    pub r#type: InferenceResultType,
    pub data: Option<InferenceToken>,
    pub message: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct LogitBias {
    pub token: i32,
    pub bias: f64,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct Generate {
    pub n_threads: i32,
    pub n_tok_predict: i32,

    /// logit bias for specific tokens
    /// Default: None
    pub logit_bias: Option<Vec<LogitBias>>,

    /// top k tokens to sample from
    /// Range: <= 0 to use vocab size
    /// Default: 40
    pub top_k: Option<i32>,

    /// top p tokens to sample from
    /// Default: 0.95
    /// 1.0 = disabled
    pub top_p: Option<f64>,

    /// tail free sampling
    /// Default: 1.0
    /// 1.0 = disabled
    pub tfs_z: Option<f64>,

    /// temperature
    /// Default: 0.80
    /// 1.0 = disabled
    pub temp: Option<f64>,

    /// locally typical sampling
    /// Default: 1.0
    /// 1.0 = disabled
    pub typical_p: Option<f64>,

    /// repeat penalty
    /// Default: 1.10
    /// 1.0 = disabled
    pub repeat_penalty: Option<f64>,

    /// last n tokens to penalize
    /// Default: 64
    /// 0 = disable penalty, -1 = context size
    pub repeat_last_n: Option<i32>,

    /// frequency penalty
    /// Default: 0.00
    /// 1.0 = disabled
    pub frequency_penalty: Option<f64>,

    /// presence penalty
    /// Default: 0.00
    /// 1.0 = disabled
    pub presence_penalty: Option<f64>,

    /// Mirostat 1.0 algorithm described in the paper https://arxiv.org/abs/2007.14966. Uses tokens instead of words.
    /// Mirostat: A Neural Text Decoding Algorithm that Directly Controls Perplexity
    /// Default: 0
    /// 0 = disabled
    /// 1 = mirostat 1.0
    /// 2 = mirostat 2.0
    pub mirostat: Option<i32>,

    /// The target cross-entropy (or surprise) value you want to achieve for the generated text. A higher value corresponds to more surprising or less predictable text, while a lower value corresponds to less surprising or more predictable text.
    /// Default: 5.0
    pub mirostat_tau: Option<f64>,

    /// The learning rate used to update `mu` based on the error between the target and observed surprisal of the sampled word. A larger learning rate will cause `mu` to be updated more quickly, while a smaller learning rate will result in slower updates.
    /// Default: 0.1
    pub mirostat_eta: Option<f64>,

    /// stop sequence
    /// Default: None
    pub stop_sequence: Option<String>,

    /// consider newlines as a repeatable token
    /// Default: true
    pub penalize_nl: Option<bool>,

    /// prompt
    pub prompt: String,
}

// Represents the configuration parameters for a LLamaContext.
#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ModelLoad {
    pub model_path: String,
    pub n_ctx: i32,
    pub n_gpu_layers: i32,
    pub seed: u32,
	pub low_vram: bool,
	pub mul_mat_q: bool,
	pub main_gpu: i32,
    pub f16_kv: bool,
    pub logits_all: bool,
    pub vocab_only: bool,
    pub use_mlock: bool,
    pub embedding: bool,
    pub use_mmap: bool,
	pub n_batch: i32,
	pub n_gqa: i32,
	pub rope_freq_base: f32,
	pub rope_freq_scale: f32,
	#[serde(skip)]
	pub tensor_split: *const f32,
	pub rms_norm_eps: f32,
    pub lora: Option<LlamaLoraAdaptor>,
}

impl Default for ModelLoad {
    fn default() -> Self {
        Self {
            model_path: "".to_string(),
            n_ctx: 2048,
            n_gpu_layers: 0,
            seed: 0,
			low_vram: false,
			mul_mat_q: false,
			main_gpu: 0,
            f16_kv: true,
            logits_all: false,
            vocab_only: false,
            use_mlock: false,
            embedding: false,
            use_mmap: true,
			rope_freq_base: 10000.0f32,
			rope_freq_scale: 1.0f32,
			tensor_split: 0.0f32,
            lora: None,
			n_batch: 512,
			n_gqa: 1,
			rms_norm_eps: 0.000005f32,
        }
    }
}

impl ModelLoad {
    // Returns the default parameters or the user-specified parameters.
    pub fn to_llama_context_params(params: &ModelLoad) -> llama_context_params {
        params.clone().into()
    }
}

impl From<ModelLoad> for llama_context_params {
    fn from(params: ModelLoad) -> Self {
        llama_context_params {
			low_vram: params.low_vram,
			main_gpu: params.main_gpu,
			mul_mat_q: params.mul_mat_q,
            n_ctx: params.n_ctx,
            n_gpu_layers: params.n_gpu_layers,
            seed: params.seed,
            f16_kv: params.f16_kv,
            logits_all: params.logits_all,
            vocab_only: params.vocab_only,
            use_mmap: params.use_mmap,
            use_mlock: params.use_mlock,
            embedding: params.embedding,
			rope_freq_base: params.rope_freq_base,
			rope_freq_scale: params.rope_freq_scale,
			tensor_split: params.tensor_split,
			n_batch: params.n_batch,
			n_gqa: params.n_gqa,
			rms_norm_eps: params.rms_norm_eps,
            progress_callback: None,
            progress_callback_user_data: std::ptr::null_mut(),
        }
    }
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct LlamaLoraAdaptor {
    pub lora_adapter: String,
    pub lora_base: Option<String>,
    pub n_threads: i32,
}
