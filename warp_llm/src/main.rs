#![deny(warnings)]

// use std::{io::Write, vec};
use clap::Parser;

use serde_derive::{Deserialize, Serialize};
// use std::sync::{Arc, Mutex};
use std::sync::Arc; // Use Arc for thread-safe reference counting
use tokio::sync::Mutex; // Use Tokio's async Mutex

use warp::Filter;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::reply::{self, Reply};
use std::fmt;

pub mod llm {
    pub mod quantized_qwen2_copy;
    pub mod llm; 
    pub mod llm_ops;
}


#[derive(Deserialize, Serialize)]
struct Prompt {
    prompt: String,
    temperature: u32,
    generated: Option<String>,
}

// Define the custom ServerError type
#[derive(Debug)]
struct ServerError {
    message: String,
}

// Implement the Reject trait for ServerError
impl Reject for ServerError {}

// Implement Display for ServerError for better error messages
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Error handler function to convert rejections into HTTP responses
async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, warp::Rejection> {
    if let Some(server_error) = err.find::<ServerError>() {
        // Return a JSON response with the error message and a 500 status code
        let json = warp::reply::json(&serde_json::json!({
            "error": server_error.message
        }));
        Ok(reply::with_status(json, StatusCode::INTERNAL_SERVER_ERROR))
    } else {
        // For other errors, return a generic 500 error
        let json = warp::reply::json(&serde_json::json!({
            "error": "Internal Server Error"
        }));
        Ok(reply::with_status(json, StatusCode::INTERNAL_SERVER_ERROR))
    }
}

#[tokio::main]
async fn main() {
    println!("Testing LLM text gen!");
    let args = llm::llm::Args::parse();
    println!("args: {:#?}", args);

    llm::llm_ops::print_setup(&args);

    // let (mut model, mut tos) = llm::llm_ops::build_model(&args).unwrap(); 
    let (model, tos) = llm::llm_ops::build_model(&args).unwrap(); 
    
    // Wrap model, tos, and args in Arc<Mutex<...>> for thread-safe sharing
    let model = Arc::new(Mutex::new(model));
    let tos = Arc::new(Mutex::new(tos));
    let args = Arc::new(args);

    let str_output = {
        let mut model = model.lock().await; // Use async lock
        let mut tos = tos.lock().await;    // Use async lock
        llm::llm_ops::run_model(&mut model, &mut tos, &args, None).unwrap()
    };
    println!("first str_output: {:#?}", str_output);

    // POST /employees/:rate  {"name":"Sean","rate":2}
    let promote = warp::post()
        .and(warp::path("generate"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(warp::any().map(move || model.clone())) // Clone Arc for each request
        .and(warp::any().map(move || tos.clone()))   // Clone Arc for each request
        .and(warp::any().map(move || args.clone()))  // Clone Arc for each request
        .and_then(
            |mut prompt: Prompt, model: Arc<Mutex<_>>, tos: Arc<Mutex<_>>, args: Arc<_>| async move {
                let mut model = model.lock().await; // Async lock
                let mut tos = tos.lock().await;     // Async lock
                match llm::llm_ops::run_model(&mut model, &mut tos, &args, Some(&prompt.prompt)) {
                    Ok(output) => {
                        prompt.generated = Some(output);
                        Ok::<_, warp::Rejection>(warp::reply::json(&prompt))
                    }
                    Err(e) => {
                        eprintln!("Error running model: {}", e);
                        Err(warp::reject::custom(ServerError {
                                message: format!("Error running model: {}", e),
                        }))
                    }
                }
            },
        );

    // Add the rejection handler to the Warp filter chain
    let routes = promote.recover(handle_rejection);

    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await
}

