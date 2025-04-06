// create a relevant subset of this candle-examples case
// https://github.com/huggingface/candle/blob/main/candle-examples/examples/quantized-qwen2-instruct/main.rs 

#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

#[cfg(feature = "accelerate")]
extern crate accelerate_src;


use clap::{Parser, ValueEnum};

use anyhow;
use tokenizers::Tokenizer;


pub const DEFAULT_PROMPT: &str = "Write a function to count prime numbers up to N. ";



#[derive(Clone, Debug, Copy, PartialEq, Eq, ValueEnum)]
pub enum Which {
    #[value(name = "0.5b")]
    W2_0_5b,
    #[value(name = "1.5b")]
    W2_1_5b,
    #[value(name = "7b")]
    W2_7b,
    #[value(name = "72b")]
    W2_72b,
    #[value(name = "deepseekr1-qwen7b")]
    DeepseekR1Qwen7B,
    #[value(name = "2.5-corder:14B")]
    W25_14b,
    #[value(name = "2.5-corder:14B-q4")]
    W25_14bQ4,
    #[value(name = "2.5-corder:14B-q8")]
    W25_14bQ8,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// GGUF file to load, typically a .gguf file generated by the quantize command from llama.cpp
    #[arg(long)]
    pub model: Option<String>,

    /// The initial prompt, use 'interactive' for entering multiple prompts in an interactive way
    /// and 'chat' for an interactive model where history of previous prompts and generated tokens
    /// is preserved.
    #[arg(long)]
    pub prompt: Option<String>,

    /// The length of the sample to generate (in tokens).
    #[arg(short = 'n', long, default_value_t = 1000)]
    pub sample_len: usize,

    /// The tokenizer config in json format.
    #[arg(long)]
    pub tokenizer: Option<String>,

    /// The temperature used to generate samples, use 0 for greedy sampling.
    #[arg(long, default_value_t = 0.8)]
    pub temperature: f64,

    /// Nucleus sampling probability cutoff.
    #[arg(long)]
    pub top_p: Option<f64>,

    /// Only sample among the top K samples.
    #[arg(long)]
    pub top_k: Option<usize>,

    /// The seed to use when generating random samples.
    #[arg(long, default_value_t = 299792458)]
    pub seed: u64,

    /// Enable tracing (generates a trace-timestamp.json file).
    #[arg(long)]
    pub tracing: bool,

    /// Process prompt elements separately.
    #[arg(long)]
    pub split_prompt: bool,

    /// Run on CPU rather than GPU even if a GPU is available.
    #[arg(long)]
    pub cpu: bool,

    /// Penalty to be applied for repeating tokens, 1. means no penalty.
    #[arg(long, default_value_t = 1.1)]
    pub repeat_penalty: f32,

    /// The context size to consider for the repeat penalty.
    #[arg(long, default_value_t = 64)]
    pub repeat_last_n: usize,

    /// The model size to use.
    #[arg(long, default_value = "0.5b")]
    pub which: Which,
}

impl Args {
    pub fn tokenizer(&self) -> anyhow::Result<Tokenizer> {
        let tokenizer_path = match &self.tokenizer {
            Some(config) => std::path::PathBuf::from(config),
            None => {
                let api = hf_hub::api::sync::Api::new()?;
                let repo = match self.which {
                    Which::W2_0_5b => "Qwen/Qwen2-0.5B-Instruct",
                    Which::W2_1_5b => "Qwen/Qwen2-1.5B-Instruct",
                    Which::W2_7b => "Qwen/Qwen2-7B-Instruct",
                    Which::W2_72b => "Qwen/Qwen2-72B-Instruct",
                    Which::DeepseekR1Qwen7B => "deepseek-ai/DeepSeek-R1-Distill-Qwen-7B",
                    Which::W25_14bQ4 => "Qwen/Qwen2.5-Coder-14B-Instruct",
                    Which::W25_14bQ8 => "Qwen/Qwen2.5-Coder-14B-Instruct",
                    Which::W25_14b => "Qwen/Qwen2.5-Coder-14B-Instruct",
                };
                let api = api.model(repo.to_string());
                api.get("tokenizer.json")?
            }
        };
        Tokenizer::from_file(tokenizer_path).map_err(anyhow::Error::msg)
    }

    pub fn model(&self) -> anyhow::Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let model_path = match &self.model {
            Some(config) => std::path::PathBuf::from(config),
            None => {
                let (repo, filename, revision) = match self.which {
                    Which::W2_0_5b => (
                        "Qwen/Qwen2-0.5B-Instruct-GGUF",
                        "qwen2-0_5b-instruct-q4_0.gguf",
                        "main",
                    ),
                    Which::W2_1_5b => (
                        "Qwen/Qwen2-1.5B-Instruct-GGUF",
                        "qwen2-1_5b-instruct-q4_0.gguf",
                        "main",
                    ),
                    Which::W2_7b => (
                        "Qwen/Qwen2-7B-Instruct-GGUF",
                        "qwen2-7b-instruct-q4_0.gguf",
                        "main",
                    ),
                    Which::W2_72b => (
                        "Qwen/Qwen2-72B-Instruct-GGUF",
                        "qwen2-72b-instruct-q4_0.gguf",
                        "main",
                    ),
                    Which::DeepseekR1Qwen7B => (
                        "unsloth/DeepSeek-R1-Distill-Qwen-7B-GGUF",
                        "DeepSeek-R1-Distill-Qwen-7B-Q4_K_M.gguf",
                        "main",
                    ),
                    Which::W25_14bQ4 => (
                        "Qwen/Qwen2.5-Coder-14B-Instruct-GGUF",
                        "qwen2.5-coder-14b-instruct-q4_0.gguf",
                        "main",
                    ),
                    Which::W25_14bQ8 => (
                        "Qwen/Qwen2.5-Coder-14B-Instruct-GGUF",
                        "qwen2.5-coder-14b-instruct-q8_0.gguf",
                        "main",
                    ),                    
                    Which::W25_14b => (
                        "Qwen/Qwen2.5-Coder-14B-Instruct-GGUF",
                        "qwen2.5-coder-14b-instruct-fp16.gguf",
                        "main",
                    ),      
                };
                let api = hf_hub::api::sync::Api::new()?;
                api.repo(hf_hub::Repo::with_revision(
                    repo.to_string(),
                    hf_hub::RepoType::Model,
                    revision.to_string(),
                ))
                .get(filename)?
            }
        };
        Ok(model_path)
    }
}

pub fn format_size(size_in_bytes: usize) -> String {
    if size_in_bytes < 1_000 {
        format!("{}B", size_in_bytes)
    } else if size_in_bytes < 1_000_000 {
        format!("{:.2}KB", size_in_bytes as f64 / 1e3)
    } else if size_in_bytes < 1_000_000_000 {
        format!("{:.2}MB", size_in_bytes as f64 / 1e6)
    } else {
        format!("{:.2}GB", size_in_bytes as f64 / 1e9)
    }
}

