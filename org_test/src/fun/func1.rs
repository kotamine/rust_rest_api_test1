use crate::utils; 
use super::super::utils2;

pub fn function() {
    println!("called func1::`function()`");
    utils::function();
    utils2::function();
}

pub fn function2() {
    println!("called func2::`function2()`");
}

