//use warp::Filter;
use super::models;


// A function to handle GET requests at /posts/{id}
pub async fn get_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    // For simplicity, let's say we are returning a static post
    let post = models::Post {
        id,
        title: String::from("Hello, Warp!"),
        body: String::from("This is a post about Warp."),
    };
    Ok(warp::reply::json(&post))
}

pub async fn generate(text: String) -> Result<impl warp::Reply, warp::Rejection> {
    // For simplicity, let's say we are returning a static post
    let post = models::Post2::new(text).generate();
    Ok(warp::reply::json(&post))
}



