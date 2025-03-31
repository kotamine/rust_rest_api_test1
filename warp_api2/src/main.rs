// a modified version of 
// https://github.com/seanmonstar/warp/blob/master/examples/body.rs 

#![deny(warnings)]

use serde_derive::{Deserialize, Serialize};

use warp::Filter;

#[derive(Deserialize, Serialize)]
struct Prompt {
    prompt: String,
    temperature: u32,
    generated: Option<String>,
}

#[tokio::main]
async fn main() {
    // pretty_env_logger::init();

    // POST /employees/:rate  {"name":"Sean","rate":2}
    let promote = warp::post()
        .and(warp::path("generate"))
        //.and(warp::path::param::<u32>())
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|mut prompt: Prompt| {
            prompt.generated = Some(String::from(
                "This is a generated text from prompt = {&prompt.prompt}."));
            warp::reply::json(&prompt)
        });
    
    println!("Server started at http://localhost:8000");
    warp::serve(promote).run(([127, 0, 0, 1], 8000)).await
}


