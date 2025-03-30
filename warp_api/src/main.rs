pub mod func {
    pub mod routes;
}
//use crate::routes::routes::routes;
// use crate::routes::routes::function;

// #[tokio::main]
// async fn main() {
//     func::routes::routes(); 
    // let routes = routes::routes::routes();
    // println!("Server started at http://localhost:8000");
    // warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
// }

fn main() {
    func::routes::routes(); 
}

