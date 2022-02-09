//use std::env;
mod module_mult;

fn main() {
    let i = [2,5,7];
    let answer = module_mult::function(i[0]);
    println!("answer = {}",answer);

}
