//use std::env;
//mod module_mult;
mod mod_sort;

fn main() {
    //let mut array = vec![2,3,8,7,99,62,0,5,5,32];
    let mut array = [2,3,8,7,99,62,0,5,5,32]; 
   
    
    //mod_sort::merge_sort(&mut array);
    mod_sort::merge_sort(&mut array[..]);

    //表示
    for elem in &array[..] {
        println!("answer = {}",*elem );
    }
   
}
