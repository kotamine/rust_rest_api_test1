
use std::{io::Write, vec};
use clap::Parser;

pub mod llm {
    pub mod quantized_qwen2_copy;
    pub mod llm; 
    pub mod llm_ops;
}


fn main() {
    println!("Testing LLM text gen!");
    let args = llm::llm::Args::parse();
    println!("args: {:#?}", args);

    llm::llm_ops::print_setup(&args);

    let (mut model, mut tos) = llm::llm_ops::build_model(&args).unwrap(); 
    
    let str_output = llm::llm_ops::run_model(&mut model, &mut tos, &args).unwrap();
    
    println!("str_output: {:#?}", str_output);

}

