//use std::env;

fn main() {
    let i = 6;
    let answer = function(i);
    println!("answer = {}",answer);

}

fn function(x:i32) -> i32{
    let fx = x * x;
    // println!("answer = {}",fx);
    return fx;
}