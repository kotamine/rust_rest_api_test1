use crate::utils;
use crate::fun::func1;
use super::utils::function3;


pub fn function() {
    println!("called utils2::`function()`");
    utils::function();
    func1::function2();
    utils::function3();
}

