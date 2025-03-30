// use warp::Filter;

// use crate::routes::handlers;
// use super::handlers;
//use crate::routes::handlers;
// use super::util::function;

// use super::utils1::function;

// use crate::func::util::function;


    // A function to build our routes
// pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     get_post()
// }

// A route to handle GET requests for a specific post
// fn get_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("posts" / u64)
//         .and(warp::get())
//         .and_then(handlers::get_post)
// }

pub fn routes() {
    println!("called routes::`routes()`");
    // handlers::get_post();
    // function();
}



