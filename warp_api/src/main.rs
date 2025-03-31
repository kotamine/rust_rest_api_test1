pub mod func {
    pub mod routes;
    // pub mod utils1;
    pub mod handlers;
    pub mod models;
}


#[tokio::main]
async fn main() {
    let routes = func::routes::routes();
    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    let routes2 = func::routes::routes2();
    warp::serve(routes2).run(([127, 0, 0, 1], 8000)).await;
}




