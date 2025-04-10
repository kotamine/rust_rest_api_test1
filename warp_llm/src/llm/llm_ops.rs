

use std::{io::Write, io::Result, vec};

use candle_core::quantized::gguf_file;
use candle_examples::token_output_stream::TokenOutputStream as TokenOutputStream;
use candle_core::Tensor;
use candle_transformers::generation::{LogitsProcessor, Sampling};

use super::llm as llm;
use super::llm::Args as Args;
use super::quantized_qwen2_copy::ModelWeights as Qwen2; 
// use super::quantized_qwen2_copy::LayerWeights;

pub fn print_setup(args: &Args) {
    println!(
        "avx: {}, neon: {}, simd128: {}, f16c: {}",
        candle_core::utils::with_avx(),
        candle_core::utils::with_neon(),
        candle_core::utils::with_simd128(),
        candle_core::utils::with_f16c()
    );
    println!(
        "temp: {:.2} repeat-penalty: {:.2} repeat-last-n: {}",
        args.temperature, args.repeat_penalty, args.repeat_last_n
    );
} 


pub fn build_model(args: &Args) -> Result<(Qwen2, TokenOutputStream)> {
    let model_path = args.model().unwrap();
    let mut file = std::fs::File::open(&model_path).unwrap();
    let start = std::time::Instant::now();
    let device = candle_examples::device(args.cpu).unwrap();

    let model = {
        let model = gguf_file::Content::read(&mut file).map_err(|e| e.with_path(model_path)).unwrap();
        let mut total_size_in_bytes = 0;
        for (_, tensor) in model.tensor_infos.iter() {
            let elem_count = tensor.shape.elem_count();
            total_size_in_bytes +=
                elem_count * tensor.ggml_dtype.type_size() / tensor.ggml_dtype.block_size();
        }
        println!(
            "loaded {:?} tensors ({}) in {:.2}s",
            model.tensor_infos.len(),
            llm::format_size(total_size_in_bytes),
            start.elapsed().as_secs_f32(),
        );
        Qwen2::from_gguf(model, &mut file, &device).unwrap()
    };
    println!("model built");

    let tokenizer = args.tokenizer().unwrap();
    let tos = TokenOutputStream::new(tokenizer);

    Ok((model, tos))
}



pub fn run_model(model: &mut Qwen2, tos: &mut TokenOutputStream, args: &Args, prompt:Option<&String>) 
    -> Result<String> {
    let device = candle_examples::device(args.cpu).unwrap();
    // let prompt_str = match prompt {
    //     Some(p) => prompt.clone(),
    //     // None => Some(&args.prompt.clone()
    //     //                 .unwrap_or_else(|| llm::DEFAULT_PROMPT.to_string())),
    //     None => { 
    //         let p = args.prompt.clone().unwrap_or_else(|| llm::DEFAULT_PROMPT.to_string());
    //         Some(&p)
    //     }
    // };
    let prompt_str0 = args.prompt.clone().unwrap_or_else(|| llm::DEFAULT_PROMPT.to_string());
    let prompt_str = if let Some(p) = prompt {
        p.clone()
    } else {
        prompt_str0.clone()
    };
    let prompt_str = match args.which {
        llm::Which::DeepseekR1Qwen7B => format!("<｜User｜>{prompt_str}<｜Assistant｜>"),
        _ => format!("<|im_start|>user\n{prompt_str}<|im_end|>\n<|im_start|>assistant\n"),
    };
    print!("formatted instruct prompt: {}", &prompt_str);

    let tokens = tos
        .tokenizer()
        .encode(prompt_str, true)
        .map_err(anyhow::Error::msg).unwrap();
    let tokens = tokens.get_ids();
    let to_sample = args.sample_len.saturating_sub(1);
    let mut all_tokens = vec![];
    let mut logits_processor = {
        let temperature = args.temperature;
        let sampling = if temperature <= 0. {
            Sampling::ArgMax
        } else {
            match (args.top_k, args.top_p) {
                (None, None) => Sampling::All { temperature },
                (Some(k), None) => Sampling::TopK { k, temperature },
                (None, Some(p)) => Sampling::TopP { p, temperature },
                (Some(k), Some(p)) => Sampling::TopKThenTopP { k, p, temperature },
            }
        };
        LogitsProcessor::from_sampling(args.seed, sampling)
    };
    let start_prompt_processing = std::time::Instant::now();
    let mut next_token = if !args.split_prompt {
        let input = Tensor::new(tokens, &device).unwrap().unsqueeze(0).unwrap();
        let logits = model.forward(&input, 0).unwrap();
        let logits = logits.squeeze(0).unwrap();
        logits_processor.sample(&logits).unwrap()
    } else {
        let mut next_token = 0;
        for (pos, token) in tokens.iter().enumerate() {
            let input = Tensor::new(&[*token], &device).unwrap().unsqueeze(0).unwrap();
            let logits = model.forward(&input, pos).unwrap();
            let logits = logits.squeeze(0).unwrap();
            next_token = logits_processor.sample(&logits).unwrap()
        }
        next_token
    };
    let prompt_dt = start_prompt_processing.elapsed();
    all_tokens.push(next_token);
    let mut str_output = String::from("");
    if let Some(t) = tos.next_token(next_token).unwrap() {
        print!("{t}");
        str_output += &t; 
        std::io::stdout().flush().unwrap();
    }

    let eos_token = match args.which {
        llm::Which::DeepseekR1Qwen7B => "<｜end▁of▁sentence｜>",
        _ => "<|im_end|>",
    };

    let eos_token = *tos.tokenizer().get_vocab(true).get(eos_token).unwrap();
    let start_post_prompt = std::time::Instant::now();

    let mut sampled = 0;
    for index in 0..to_sample {
        // in the backend, the model is using cache to add next token. 
        // try clearing the kv_cache for all layers
        // for layer in model.layers.iter_mut() {
        //     layer.kv_cache = None; 
        // }  

        let input = Tensor::new(&[next_token], &device).unwrap().unsqueeze(0).unwrap();
        let logits: Tensor = model.forward(&input, tokens.len() + index).unwrap();
        let logits = logits.squeeze(0).unwrap();
        let logits = if args.repeat_penalty == 1. {
            logits
        } else {
            let start_at = all_tokens.len().saturating_sub(args.repeat_last_n);
            candle_transformers::utils::apply_repeat_penalty(
                &logits,
                args.repeat_penalty,
                &all_tokens[start_at..],
            ).unwrap()
        };
        next_token = logits_processor.sample(&logits).unwrap();
        all_tokens.push(next_token);
        if let Some(t) = tos.next_token(next_token).unwrap() {
            print!("{t}");
            str_output += &t; 
            std::io::stdout().flush().unwrap();
        }
        sampled += 1;
        if next_token == eos_token {
            break;
        };
    }
    if let Some(rest) = tos.decode_rest().map_err(candle_core::Error::msg).unwrap() {
        print!("{rest}");
        str_output += &rest; 
    }
    std::io::stdout().flush().unwrap();
    let dt = start_post_prompt.elapsed();
    println!(
        "\n\n{:4} prompt tokens processed: {:.2} token/s",
        tokens.len(),
        tokens.len() as f64 / prompt_dt.as_secs_f64(),
    );
    println!(
        "{sampled:4} tokens generated: {:.2} token/s",
        sampled as f64 / dt.as_secs_f64(),
    );

    // clear the kv_cache for all layers
    for layer in model.layers.iter_mut() {
            layer.kv_cache = None; 
    }

    Ok(str_output)
}


