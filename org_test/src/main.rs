//use crate::utils;
mod utils;
mod utils2;
pub mod fun {
    pub mod func1;
}

fn main() {
    println!("Hello, world!");
    utils::function();
    println!("----------------");
    utils2::function();
    println!("----------------");
    fun::func1::function();
    println!("----------------");
}

